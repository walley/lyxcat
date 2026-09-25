use clap_mangen::Man;
use std::path::Path;

fn main() {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    let cmd = clap::Command::new("lyxcat")
        .about("View and extract text from LyX (.lyx) documents")
        .long_about("lyxcat is a command-line tool to view and extract text from LyX (.lyx) documents.\nIt supports inline text extraction, special character processing, and note/footnote\nhandling. Originally designed as a viewer for midnight commander.")
        .author("walley <walley@walley.org>")
        .version("0.1.0")
        .arg(
            clap::Arg::new("FILE")
                .help("The LyX (.lyx) file to view")
                .required(true)
                .value_name("FILE"),
        );

    Man::new(cmd).generate_to(out_path).unwrap();

    println!("Man page generated in: {}", out_path.display());
}
