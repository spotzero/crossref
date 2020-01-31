# Crossref

CLI command to cross reference lines from one file in another file.

## Usage

```crossref source-file search-file```

Will output:

```
For each line in source-file:
  Line number in search-file: Line in search-file that contains the line from source-file
```

The line from the source file are matched as case-insensitive matches in the search file.

## Getting Started

### Prerequisites

At a minimum, you need to Rust compiler.


### Installing

```
cargo build --release
cp target/crossref "somewhere in you path/bin"
```

## Built With

* [Rust](https://www.rust-lang.org/) - Rust Language

