use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    // Get the filename argument from mc
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
                // Strip out LyX specific inline formatting
                let clean_text = inset_regex.replace_all(&paragraph, "");
                println!("{}\n", clean_text.trim());
                paragraph.clear();
            }
            continue;
        }

        if in_layout {
            // Ignore sub-properties or layout metadata lines starting with a backslash
            if !trimmed.starts_with('\\') {
                paragraph.push_str(&line);
                paragraph.push(' '); // Keep word separation across text wrappers
            }
        }
    }

    Ok(())
}

