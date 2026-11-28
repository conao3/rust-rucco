<p align="center"><img src="docs/assets/banner.png" alt="rucco: A Common Lisp interpreter written in Rust, with an interactive REPL" width="880"></p>

<p align="center"><strong>A Common Lisp interpreter written in Rust, with an interactive REPL</strong></p>

<p align="center"><a href="LICENSE"><img src="https://img.shields.io/github/license/conao3/rust-rucco" alt="License"></a> <a href="https://github.com/conao3/rust-rucco/stargazers"><img src="https://img.shields.io/github/stars/conao3/rust-rucco?style=flat" alt="GitHub stars"></a></p>

<p align="center"><a href="#installation">Installation</a> | <a href="#usage">Usage</a> | <a href="#license">License</a></p>

<h1>rucco</h1>

A Common Lisp implementation written in Rust.

## Overview

rucco is an experimental Common Lisp interpreter built from scratch in Rust. It features an interactive REPL, arena-based memory management, and a modular architecture consisting of a reader, compiler, and type system.

## Features

- Interactive REPL with command history
- Arena-based memory management for efficient allocation
- S-expression reader with support for cons cells and atoms
- Proper list and dotted pair notation

## Installation

### From Source

```sh
git clone https://github.com/conao3/rust-rucco.git
cd rust-rucco
cargo build --release
```

The compiled binary will be available at `target/release/rucco`.

## Usage

Start the REPL:

```sh
cargo run
```

Or run the compiled binary:

```sh
./target/release/rucco
```

You will see the `rucco>` prompt where you can enter Lisp expressions.

## Project Structure

```
src/
  main.rs       # REPL entry point
  lib.rs        # Library exports
  core.rs       # Read-eval-print implementation
  reader.rs     # S-expression parser
  compiler.rs   # Expression compiler
  types/        # Type definitions
    rucco_exp.rs    # Expression types (atoms, cons cells)
    rucco_atom.rs   # Atomic types
    rucco_arena.rs  # Arena allocator
    rucco_err.rs    # Error types
```

## Requirements

- Rust 2021 edition or later

## License

Apache-2.0

## Author

conao3
