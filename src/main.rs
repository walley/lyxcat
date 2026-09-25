use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;

/// lyxcat - A command-line tool to view and extract text from LyX (.lyx) documents.
///
/// lyxcat parses LyX document files and outputs their text content in a readable format.
/// It handles inline formatting, special characters, notes, and footnotes.
#[derive(Parser, Debug)]
#[command(name = "lyxcat")]
#[command(author = "walley <walley@walley.org")]
#[command(version = "0.1.0")]
#[command(about = "View and extract text from LyX (.lyx) documents")]
#[command(long_about = "lyxcat is a command-line tool to view and extract text from LyX (.lyx) documents.\nIt supports inline text extraction, special character processing, and note/footnote\nhandling. Originally designed as a viewer for midnight commander.")]
struct Args {
    /// The LyX (.lyx) file to view
    #[arg(value_name = "FILE")]
    file: PathBuf,
}

/// Layout type and its display formatting
#[derive(Debug, Clone, Copy, PartialEq)]
enum LayoutType {
    Standard,
    Title,
    Section,
    Subsection,
    Subsubsection,
    SectionStar,
    SubsectionStar,
    SubsubsectionStar,
    Enumerate,
    Itemize,
    Description,
    Quote,
    Verse,
    Center,
    FlushLeft,
    FlushRight,
    TitlePage,
    Author,
    Date,
    Abstract,
    Unknown,
}

impl LayoutType {
    fn from_name(name: &str) -> Self {
        match name {
            "Standard" => LayoutType::Standard,
            "Title" => LayoutType::Title,
            "Section" => LayoutType::Section,
            "Subsection" => LayoutType::Subsection,
            "Subsubsection" => LayoutType::Subsubsection,
            "Section*" => LayoutType::SectionStar,
            "Subsection*" => LayoutType::SubsectionStar,
            "Subsubsection*" => LayoutType::SubsubsectionStar,
            "Enumerate" => LayoutType::Enumerate,
            "Itemize" => LayoutType::Itemize,
            "Description" => LayoutType::Description,
            "Quote" => LayoutType::Quote,
            "Verse" => LayoutType::Verse,
            "Center" => LayoutType::Center,
            "FlushLeft" => LayoutType::FlushLeft,
            "FlushRight" => LayoutType::FlushRight,
            "Title Page" => LayoutType::TitlePage,
            "Author" => LayoutType::Author,
            "Date" => LayoutType::Date,
            "Abstract" => LayoutType::Abstract,
            _ => LayoutType::Unknown,
        }
    }

    fn is_list(&self) -> bool {
        matches!(
            self,
            LayoutType::Enumerate | LayoutType::Itemize | LayoutType::Description
        )
    }

    fn is_heading(&self) -> bool {
        matches!(
            self,
            LayoutType::Title
                | LayoutType::Section
                | LayoutType::Subsection
                | LayoutType::Subsubsection
                | LayoutType::SectionStar
                | LayoutType::SubsectionStar
                | LayoutType::SubsubsectionStar
        )
    }

    fn prefix(&self) -> &'static str {
        match self {
            LayoutType::Title => "\n",
            LayoutType::Section | LayoutType::SectionStar => "\n",
            LayoutType::Subsection | LayoutType::SubsectionStar => "\n",
            LayoutType::Subsubsection | LayoutType::SubsubsectionStar => "\n",
            LayoutType::TitlePage => "\n",
            LayoutType::Author => "\n",
            LayoutType::Date => "\n",
            LayoutType::Abstract => "\n",
            LayoutType::Enumerate => "\n  * ",
            LayoutType::Itemize => "\n  - ",
            LayoutType::Quote => "\n",
            LayoutType::Verse => "\n  ",
            LayoutType::Center => "\n",
            LayoutType::FlushLeft => "\n",
            LayoutType::FlushRight => "\n",
            LayoutType::Description => "\n",
            _ => "",
        }
    }

    fn suffix(&self) -> &'static str {
        match self {
            LayoutType::Author => "",
            LayoutType::Abstract => "",
            _ => "",
        }
    }
}

/// Inset types and their handling
#[derive(Debug, Clone, Copy, PartialEq)]
enum InsetType {
    Note,
    Foot,
    Quotes,
    FlexUrl,
    Newline,
    Space,
    Info,
    CommandInset,
    Href,
    Unknown,
}

impl InsetType {
    fn from_line(line: &str) -> Option<Self> {
        if line.contains("\\begin_inset Note") {
            Some(InsetType::Note)
        } else if line.contains("\\begin_inset Foot") {
            Some(InsetType::Foot)
        } else if line.contains("\\begin_inset Quotes") {
            Some(InsetType::Quotes)
        } else if line.contains("\\begin_inset Flex URL") {
            Some(InsetType::FlexUrl)
        } else if line.contains("\\begin_inset Newline") {
            Some(InsetType::Newline)
        } else if line.contains("\\begin_inset space") {
            Some(InsetType::Space)
        } else if line.contains("\\begin_inset Info") {
            Some(InsetType::Info)
        } else if line.contains("\\begin_inset CommandInset") {
            Some(InsetType::CommandInset)
        } else if line.contains("\\begin_inset href") {
            Some(InsetType::Href)
        } else {
            None
        }
    }
}

fn process_paragraph(
    paragraph: &str,
    special_char_regex: &Regex,
    special_char_map: &HashMap<&str, &str>,
    inset_regex: &Regex,
) -> String {
    // Process and format \SpecialChar instances
    let processed_special = special_char_regex.replace_all(paragraph, |caps: &regex::Captures| {
        let raw_match = &caps[1];
        if let Some(&replacement) = special_char_map.get(raw_match) {
            replacement.to_string()
        } else {
            let clean_name = raw_match
                .trim_start_matches('\\')
                .trim_end_matches("{}");
            format!("[{}]", clean_name)
        }
    });

    // Clean up structural inline formatting insets
    let clean_text: String = inset_regex.replace_all(&processed_special, "").into();
    clean_text
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file_path = &args.file;

    let file = File::open(file_path)
        .with_context(|| format!("Failed to open file: {}", file_path.display()))?;
    let reader = BufReader::new(file);

    // Special character mappings (LyX SpecialChar -> plain text)
    let special_char_map: HashMap<&str, &str> = [
        ("menuseparator", ">"),
        ("softhyphen", "-"),
        ("LyX", "LyX"),
        ("LaTeX", "LaTeX"),
        ("TeX", "TeX"),
        ("\\ldots{}", "..."),
        ("\\ldots", "..."),
        ("em dash", "--"),
        ("en dash", "-"),
        ("leftarrow", "<- "),
        ("rightarrow", " ->"),
        ("le", "<="),
        ("ge", ">="),
        ("neq", "!="),
        ("approx", "~="),
        ("pm", "+"),
        ("times", "x"),
        ("div", "/"),
        ("copyright", "(c)"),
        ("registered", "(R)"),
        ("trademark", "TM"),
    ]
    .iter()
    .cloned()
    .collect();

    // Regex patterns
    let inset_regex = Regex::new(r"\\inset [^}]+}")
        .context("Failed to compile inset regex")?;
    let special_char_regex = Regex::new(r"\\SpecialChar\s+([^\\\s]+)")
        .context("Failed to compile special_char regex")?;
    let note_regex = Regex::new(r"\\begin_inset Note")
        .context("Failed to compile note regex")?;
    let footnote_regex = Regex::new(r"\\begin_inset Foot")
        .context("Failed to compile footnote regex")?;
    let layout_regex = Regex::new(r"\\begin_layout\s+(.+)")
        .context("Failed to compile layout regex")?;
    let math_regex = Regex::new(r"\\begin_math|\\end_math")
        .context("Failed to compile math regex")?;
    let display_math_regex = Regex::new(r"\\begin_display_math|\\end_display_math")
        .context("Failed to compile display_math regex")?;
    let quotes_eld_regex = Regex::new(r"\\begin_inset Quotes eld")
        .context("Failed to compile quotes_eld regex")?;
    let quotes_erd_regex = Regex::new(r"\\begin_inset Quotes erd")
        .context("Failed to compile quotes_erd regex")?;

    // State tracking
    let mut in_layout = false;
    let mut current_layout = LayoutType::Unknown;
    let mut in_note = false;
    let mut in_footnote = false;
    let mut in_math = false;
    let mut in_display_math = false;
    let mut in_inset = false;
    let mut inset_type = InsetType::Unknown;

    let mut paragraph = String::new();
    let mut notes: Vec<String> = Vec::new();
    let mut footnotes: Vec<String> = Vec::new();
    let mut note_counter = 0;
    let mut footnote_counter = 0;
    let mut note_content = String::new();
    let mut footnote_content = String::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();

        // Handle inset beginnings
        if trimmed.starts_with("\\begin_inset") {
            in_inset = true;
            if let Some(it) = InsetType::from_line(trimmed) {
                inset_type = it;
                match inset_type {
                    InsetType::Note => {
                        in_note = true;
                        note_counter += 1;
                        paragraph.push_str(&format!("[note {}]", note_counter));
                        note_content.clear();
                    }
                    InsetType::Foot => {
                        in_footnote = true;
                        footnote_counter += 1;
                        paragraph.push_str(&format!("[footnote {}]", footnote_counter));
                        footnote_content.clear();
                    }
                    InsetType::Quotes => {
                        paragraph.push('"');
                    }
                    InsetType::FlexUrl => {
                        // URLs will be handled by their content
                    }
                    InsetType::Newline => {
                        paragraph.push('\n');
                    }
                    InsetType::Space => {
                        paragraph.push(' ');
                    }
                    InsetType::Info | InsetType::CommandInset | InsetType::Href => {
                        // Skip info and command insets, but keep href content
                    }
                    InsetType::Unknown => {}
                }
            }
            continue;
        }

        // Handle inset endings
        if trimmed.starts_with("\\end_inset") {
            in_inset = false;
            match inset_type {
                InsetType::Note => {
                    in_note = false;
                    if !note_content.is_empty() {
                        notes.push(format!("Note{}: {}", note_counter, note_content.trim()));
                    }
                    note_content.clear();
                }
                InsetType::Foot => {
                    in_footnote = false;
                    if !footnote_content.is_empty() {
                        footnotes.push(format!("Footnote{}: {}", footnote_counter, footnote_content.trim()));
                    }
                    footnote_content.clear();
                }
                InsetType::Quotes => {
                    paragraph.push('"');
                }
                InsetType::FlexUrl => {
                    // URL inset ended
                }
                InsetType::Href => {
                    // href inset ended
                }
                _ => {}
            }
            inset_type = InsetType::Unknown;
            continue;
        }

        // Handle quote markers specifically
        if quotes_eld_regex.is_match(trimmed) || quotes_erd_regex.is_match(trimmed) {
            continue;
        }

        // Handle note insets (legacy, for nested content)
        if !in_inset && note_regex.is_match(trimmed) {
            in_note = true;
            note_counter += 1;
            paragraph.push_str(&format!("[note {}]", note_counter));
            note_content.clear();
            continue;
        }

        if in_note && !in_inset {
            if trimmed.starts_with("\\end_inset") {
                in_note = false;
                if !note_content.is_empty() {
                    notes.push(format!("Note{}: {}", note_counter, note_content.trim()));
                }
                note_content.clear();
                continue;
            }

            if trimmed.starts_with("\\begin_layout") {
                continue;
            } else if trimmed.starts_with("\\end_layout") {
                if !note_content.is_empty() {
                    note_content.push(' ');
                }
                continue;
            }

            if !trimmed.starts_with('\\') || trimmed.contains("\\SpecialChar") {
                note_content.push_str(&line);
                note_content.push(' ');
            }
            continue;
        }

        // Handle footnote insets (legacy)
        if !in_inset && footnote_regex.is_match(trimmed) {
            in_footnote = true;
            footnote_counter += 1;
            paragraph.push_str(&format!("[footnote {}]", footnote_counter));
            footnote_content.clear();
            continue;
        }

        if in_footnote && !in_inset {
            if trimmed.starts_with("\\end_inset") {
                in_footnote = false;
                if !footnote_content.is_empty() {
                    footnotes.push(format!("Footnote{}: {}", footnote_counter, footnote_content.trim()));
                }
                footnote_content.clear();
                continue;
            }

            if trimmed.starts_with("\\begin_layout") {
                continue;
            } else if trimmed.starts_with("\\end_layout") {
                if !footnote_content.is_empty() {
                    footnote_content.push(' ');
                }
                continue;
            }

            if !trimmed.starts_with('\\') || trimmed.contains("\\SpecialChar") {
                footnote_content.push_str(&line);
                footnote_content.push(' ');
            }
            continue;
        }

        // Handle math mode
        if math_regex.is_match(trimmed) {
            if trimmed.starts_with("\\begin_math") {
                in_math = true;
                paragraph.push('$');
            } else if trimmed.starts_with("\\end_math") {
                in_math = false;
                paragraph.push('$');
            }
            continue;
        }

        if display_math_regex.is_match(trimmed) {
            if trimmed.starts_with("\\begin_display_math") {
                in_display_math = true;
                paragraph.push_str("\n$$");
            } else if trimmed.starts_with("\\end_display_math") {
                in_display_math = false;
                paragraph.push_str("$$\n");
            }
            continue;
        }

        // Handle layout changes
        if let Some(caps) = layout_regex.captures(trimmed) {
            let layout_name = &caps[1];
            current_layout = LayoutType::from_name(layout_name);
            in_layout = true;

            // For list items, start fresh
            if current_layout.is_list() {
                // Process the current paragraph before starting new layout
                if !paragraph.is_empty() {
                    let processed = process_paragraph(&paragraph, &special_char_regex, &special_char_map, &inset_regex);
                    println!("{}\n", processed.trim());
                    paragraph.clear();
                }
                // Add layout prefix
                paragraph.push_str(current_layout.prefix());
            }
            continue;
        }

        if trimmed.starts_with("\\end_layout") {
            in_layout = false;
            if !paragraph.is_empty() {
                // Add layout suffix
                paragraph.push_str(current_layout.suffix());

                let processed = process_paragraph(&paragraph, &special_char_regex, &special_char_map, &inset_regex);
                println!("{}\n", processed.trim());
                paragraph.clear();
            }
            current_layout = LayoutType::Unknown;
            continue;
        }

        // Collect content
        if in_layout && !in_inset && !in_math && !in_display_math {
            if !trimmed.starts_with('\\') || trimmed.contains("\\SpecialChar") {
                paragraph.push_str(&line);
                paragraph.push(' ');
            }
        }
    }

    // Flush any remaining paragraph
    if !paragraph.is_empty() {
        let processed = process_paragraph(&paragraph, &special_char_regex, &special_char_map, &inset_regex);
        println!("{}\n", processed.trim());
    }

    // Print all notes and footnotes at the end
    if !notes.is_empty() || !footnotes.is_empty() {
        println!();
        for note in notes {
            println!("{}", note);
        }
        for footnote in footnotes {
            println!("{}", footnote);
        }
    }

    Ok(())
}
