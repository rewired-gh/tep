# tep

[![PR Welcome](https://img.shields.io/badge/PR-Welcome-yellow)](https://github.com/h0gan1ee/tep/pulls) [![crates.io](https://img.shields.io/crates/v/tep)](https://crates.io/crates/tep) [![downloads](https://img.shields.io/crates/d/tep)](https://crates.io/crates/tep) [![release](https://img.shields.io/github/v/release/h0gan1ee/tep)](https://github.com/h0gan1ee/tep/releases/latest) [![build](https://img.shields.io/github/workflow/status/h0gan1ee/tep/Rust)](https://github.com/h0gan1ee/tep/actions/workflows/rust.yml)

[Rust package on crates.io](https://crates.io/crates/tep)

A blazingly fast command-line tool for converting Chinese punctuations **t**o **E**nglish **p**unctuations.

<img width="800" alt="Screen Shot" src="https://user-images.githubusercontent.com/39949564/153745648-b667e6cd-5c45-481e-be69-ebae93fc7558.png">

## Installation

```sh
cargo install tep
```

## Command-line Usage

### Usage Overview

```
USAGE:
    tep [FLAGS] <input> [output]

FLAGS:
    -h, --help       Prints help information
    -t, --trim       Trim empty character(s) for each line and the content
    -V, --version    Prints version information

ARGS:
    <input>     Input file
    <output>    Output file, same as input file if not present
```

- `<>` means the argument is required, while `[]` means optional.
