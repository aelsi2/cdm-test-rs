# Compliance tests for CDM-16 Rust

This repository contains a test suite for the [experimental CdM-16 Rust compiler](https://github.com/ylab-nsu/cdm16-rust) based on the [CdM-16 LLVM backend](https://github.com/ylab-nsu/cdm16-llvm-neo/).
It compiles single-source programs and executes them on the [cocoemu debug server](https://github.com/cdm-processors/cdm-devkit/tree/master/cocoemu-server).

Directories:
- `test-runner` - the test runner that runs on the host machine
- `test-base` - the base project where test files are included
- `tests` - the tests themselves

## How to run
```sh
cargo run -- -c ~/.rust-cdm/ -t test-base tests
```

`~/.rust-cdm/` should be replaced with the location where you have the toolchain installed.
`cocoemu-server` must be available in `$PATH`.
