use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: lyxcat <file.lyx>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Regex to remove basic inline formatting like \inset ... }
    let inset_regex = Regex::new(r"\\inset [^}]+}").unwrap();
    
    // Case-sensitive regex targeting \SpecialChar followed by an identifier or a command
    let special_char_regex = Regex::new(r"\\SpecialChar\s+(\\[a-zA-Z]+(?:\{\})?|[a-zA-Z0-9_-]+)").unwrap();

    // Regex to detect note and footnote insets
    let note_regex = Regex::new(r"\\begin_inset Note").unwrap();
    let footnote_regex = Regex::new(r"\\begin_inset Foot").unwrap();

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
                    
                    // Match based on raw token appearance in the file
                    match raw_match {
                        "menuseparator" => "[>]".to_string(),
                        "\\ldots{}" | "\\ldots" => "...".to_string(),
                        _ => {
                            // Strip leading backslash and trailing brackets for any other LaTeX-style commands
                            let clean_name = raw_match
                                .trim_start_matches('\\')
                                .trim_end_matches("{}");
                            format!("[{}]", clean_name)
                        }
                    }
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
