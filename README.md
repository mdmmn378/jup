# Jup

## Description

Jup is a tool that converts Jupyter notebooks to Markdown files. It's built with Rust and provides a simple and efficient way to transform your notebooks into a more version-control friendly format.

## Installation

1. Clone the repo

```sh
git clone git@github.com:mdmmn378/jup.git
```

2. Build the project

```sh
cargo build
```

## Usage

Run the program to generate markdown in std output:

```sh
jup notebook.ipynb
```

Add `--write` flag to write to a file

```sh
jup --write notebook.ipynb
```

The output should be `notebook.md`
