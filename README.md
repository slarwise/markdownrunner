# README

Run code snippets in markdown, similar to runme. Make it possible to edit the
code before running it.

## Services

- Extract code blocks
- Run code block
- TUI for selecting the code block

## Usage

Extract code blocks using the cli:

```sh
cargo run -- extract <path-to-file>
```

Launch server on localhost:3000 and extract code blocks:

```sh
cargo run -- serve &
curl --data-binary @<path-to-file> http://localhost:3000/extract
```
