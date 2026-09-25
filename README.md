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

### Overview

lyxcat was born from a simple idea: LyX documents (`.lyx` files) contain rich text content, but there was no easy way to quickly preview them from the command line or within file managers. While LyX itself is a powerful document processor, sometimes you just want to read the text without launching the full application.

### Technical Details

- **Language**: Rust (2021 Edition)
- **License**: AGPL-3.0+
- **Author**: walley <walley@walley.org>
- **Repository**: https://github.com/walley/lyxcat
- **Issue Tracker**: https://github.com/walley/lyxcat/issues

### Dependencies

- **Runtime**: None (statically compiled binary)
- **Build**: Rust toolchain (cargo, rustc)
- **Debian Packaging**: debhelper, dpkg-dev, cargo, rustc

### Platform Support

lyxcat is designed to work on any platform where Rust compiles:

- **Linux** (primary target, Debian packages available)
- **macOS** (via `cargo install`)
- **Windows** (via `cargo install`, experimental)
- **BSD** (via `cargo install`)

The Debian package is built for `amd64` architecture. Other architectures can be built from source.

### Versioning

lyxcat follows semantic versioning (SemVer) principles:
- `MAJOR` version for breaking changes
- `MINOR` version for new features
- `PATCH` version for bug fixes

### Related Projects

- [LyX](https://www.lyx.org/) - The document processor that lyxcat reads
- [pandoc](https://pandoc.org/) - Universal document converter (supports LyX via LaTeX)
- [midnight commander](https://midnight-commander.org/) - File manager where lyxcat is integrated

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
