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
#[command(author = "walley <walley@walley.org>")]
#[command(version = "0.1.0")]
#[command(about = "View and extract text from LyX (.lyx) documents")]
#[command(long_about = "lyxcat is a command-line tool to view and extract text from LyX (.lyx) documents.\nIt supports inline text extraction, special character processing, and note/footnote\nhandling. Originally designed as a viewer for midnight commander.")]
struct Args {
    /// The LyX (.lyx) file to view
    #[arg(value_name = "FILE")]
    file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file_path = &args.file;

    let file = File::open(file_path)
        .with_context(|| format!("Failed to open file: {}", file_path.display()))?;
    let reader = BufReader::new(file);

    // Special character mappings (LyX SpecialChar -> plain text)
    let special_char_map: std::collections::HashMap<&str, &str> = [
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

    // Regex to remove basic inline formatting like \inset ... }
    let inset_regex = Regex::new(r"\\inset [^}]+}")
        .context("Failed to compile inset regex")?;

    // Regex to match \SpecialChar followed by identifier or command
    let special_char_regex = Regex::new(r"\\SpecialChar\s+([^\\\s]+)")
        .context("Failed to compile special_char regex")?;

    // Regex to detect note and footnote insets
    let note_regex = Regex::new(r"\\begin_inset Note")
        .context("Failed to compile note regex")?;
    let footnote_regex = Regex::new(r"\\begin_inset Foot")
        .context("Failed to compile footnote regex")?;

    let mut in_layout = false;
    let mut in_note = false;
    let mut in_footnote = false;
    let mut paragraph = String::new();
    let mut notes: Vec<String> = Vec::new();
    let mut footnotes: Vec<String> = Vec::new();
    let mut note_counter = 0;
    let mut footnote_counter = 0;
    let mut note_content = String::new();
    let mut footnote_content = String::new();
    let mut note_in_layout = false;
    let mut footnote_in_layout = false;

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();

        // Handle note insets
        if note_regex.is_match(trimmed) {
            in_note = true;
            note_counter += 1;
            paragraph.push_str(&format!("[note {}]", note_counter));
            note_content.clear();
            continue;
        }

        if in_note {
            if trimmed.starts_with("\\end_inset") {
                in_note = false;
                if !note_content.is_empty() {
                    notes.push(format!("Note{}: {}", note_counter, note_content.trim()));
                }
                note_content.clear();
                note_in_layout = false;
                continue;
            }

            if trimmed.starts_with("\\begin_layout") {
                note_in_layout = true;
                continue;
            } else if trimmed.starts_with("\\end_layout") {
                note_in_layout = false;
                if !note_content.is_empty() {
                    note_content.push(' ');
                }
                continue;
            }

            if note_in_layout {
                if !trimmed.starts_with('\\') || trimmed.contains("\\SpecialChar") {
                    note_content.push_str(&line);
                    note_content.push(' ');
                }
            }
            continue;
        }

        // Handle footnote insets
        if footnote_regex.is_match(trimmed) {
            in_footnote = true;
            footnote_counter += 1;
            paragraph.push_str(&format!("[footnote {}]", footnote_counter));
            footnote_content.clear();
            continue;
        }

        if in_footnote {
            if trimmed.starts_with("\\end_inset") {
                in_footnote = false;
                if !footnote_content.is_empty() {
                    footnotes.push(format!("Footnote{}: {}", footnote_counter, footnote_content.trim()));
                }
                footnote_content.clear();
                footnote_in_layout = false;
                continue;
            }

            if trimmed.starts_with("\\begin_layout") {
                footnote_in_layout = true;
                continue;
            } else if trimmed.starts_with("\\end_layout") {
                footnote_in_layout = false;
                if !footnote_content.is_empty() {
                    footnote_content.push(' ');
                }
                continue;
            }

            if footnote_in_layout {
                if !trimmed.starts_with('\\') || trimmed.contains("\\SpecialChar") {
                    footnote_content.push_str(&line);
                    footnote_content.push(' ');
                }
            }
            continue;
        }

        // Main layout processing
        if trimmed.starts_with("\\begin_layout") {
            in_layout = true;
            continue;
        } else if trimmed.starts_with("\\end_layout") {
            in_layout = false;
            if !paragraph.is_empty() {
                // Process and format \SpecialChar instances
                let processed_special = special_char_regex.replace_all(&paragraph, |caps: &regex::Captures| {
                    let raw_match = &caps[1];

                    // Look up in the mapping table first
                    if let Some(&replacement) = special_char_map.get(raw_match) {
                        return replacement.to_string();
                    }

                    // Default: strip leading backslash and trailing brackets
                    let clean_name = raw_match
                        .trim_start_matches('\\')
                        .trim_end_matches("{}");
                    format!("[{}]", clean_name)
                });

                // Clean up structural inline formatting insets
                let clean_text = inset_regex.replace_all(&processed_special, "");

                println!("{}\n", clean_text.trim());
                paragraph.clear();
            }
            continue;
        }

        if in_layout {
            // Include lines if they are body text or explicitly contain inline \SpecialChar commands
            if !trimmed.starts_with('\\') || trimmed.contains("\\SpecialChar") {
                paragraph.push_str(&line);
                paragraph.push(' ');
            }
        }
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
