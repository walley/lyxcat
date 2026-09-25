# lyxcat

A command-line tool to view and extract plain text from LyX (.lyx) documents.

## Description

lyxcat is **hardcoded into midnight commander** - once installed, it will work immediately as the default viewer for `.lyx` files in mc without any additional configuration.

lyxcat parses LyX document files and outputs their text content in a human-readable format. It handles:

- Paragraphs and layouts (Title, Section, Enumerate, etc.)
- Inline formatting removal (insets)
- Special characters (LyX, LaTeX, TeX, menu separators, hyphens, etc.)
- Notes and footnotes

Originally designed as a viewer for midnight commander.

## Installation

### From Source (Cargo)

```bash
cargo install --git https://github.com/walley/lyxcat
```

Or clone and build:

```bash
git clone https://github.com/walley/lyxcat
cd lyxcat
cargo build --release
./target/release/lyxcat --help
```

### Debian Package

Download the latest `.deb` from [GitHub Releases](https://github.com/walley/lyxcat/releases) and install:

```bash
wget https://github.com/walley/lyxcat/releases/download/v0.1.1/lyxcat_0.1.0-1_amd64.deb
sudo dpkg -i lyxcat_0.1.0-1_amd64.deb
```

Once installed, **open midnight commander and navigate to any `.lyx` file - press Enter or F3 to view it with lyxcat automatically**.

## Usage

```bash
lyxcat <file.lyx>
```

### Options

```
lyxcat is a command-line tool to view and extract text from LyX (.lyx) documents.

Usage: lyxcat <FILE>

Arguments:
  <FILE>    The LyX (.lyx) file to view

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Examples

```bash
# View a LyX document
lyxcat document.lyx

# Pipe to less for paginated viewing
lyxcat document.lyx | less

# Search for text in a LyX file
lyxcat document.lyx | grep -i "important"
```

## Supported Features

### Layouts
- Standard paragraphs
- Titles, Sections, Subsections
- Enumerate (numbered lists)
- Itemize (bulleted lists)
- And more...

### Special Characters
| LyX SpecialChar | Output |
|----------------|--------|
| `LyX` | LyX |
| `LaTeX` | LaTeX |
| `TeX` | TeX |
| `menuseparator` | > |
| `softhyphen` | - |
| `\ldots` | ... |
| `em dash` | -- |
| `en dash` | - |
| `copyright` | (c) |
| `registered` | (R) |
| `trademark` | TM |
| And many more... |

### Notes and Footnotes
Notes and footnotes are extracted and displayed at the end of the output with markers like `[note 1]`, `[footnote 1]`.

## Project Info

- **License**: AGPL-3.0+
- **Author**: walley <walley@walley.org>
- **Source**: https://github.com/walley/lyxcat
- **Issues**: https://github.com/walley/lyxcat/issues

## Contributing

Pull requests welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `cargo test` (coming soon)
5. Submit a pull request

## Release Process

To create a new release:

```bash
./release.sh [version]
```

This will:
1. Update the changelog
2. Create a git tag
3. Push the tag to trigger the CI release workflow

The workflow automatically builds the Debian package and attaches it to the GitHub release.
