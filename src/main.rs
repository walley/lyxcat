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
    
    // Case-sensitive regex targeting \SpecialChar followed by an identifier or a command (e.g., \ldots{})
    let special_char_regex = Regex::new(r"\\SpecialChar\s+(\\[a-zA-Z]+(?:\{\})?|[a-zA-Z0-9_-]+)").unwrap();

    let mut in_layout = false;
    let mut paragraph = String::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();

        if trimmed.starts_with("\\begin_layout") {
            in_layout = true;
            continue;
        } else if trimmed.starts_with("\\end_layout") {
            in_layout = false;
            if !paragraph.is_empty() {
                // 1. Process and format \SpecialChar instances into [Name]
                let processed_special = special_char_regex.replace_all(&paragraph, |caps: &regex::Captures| {
                    let matched = &caps[1];
                    // Strip leading backslash and trailing brackets if it's a LaTeX-style command (like \ldots{})
                    let clean_name = matched
                        .trim_start_matches('\\')
                        .trim_end_matches("{}");
                    format!("[{}]", clean_name)
                });

                // 2. Clean up structural inline formatting insets
                let clean_text = inset_regex.replace_all(&processed_special, "");
                
                println!("{}\n", clean_text.trim());
                paragraph.clear();
            }
            continue;
        }

        if in_layout {
            // Ignore sub-properties or layout metadata lines starting with a backslash
            // BUT allow the line if it specifically contains our inline \SpecialChar command
            if !trimmed.starts_with('\\') || trimmed.contains("\\SpecialChar") {
                paragraph.push_str(&line);
                paragraph.push(' ');
            }
        }
    }

    Ok(())
}

