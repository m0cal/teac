# Teac

A Rust-based compiler for the TeaLang (Teaching Programming Language), featuring LLVM IR generation and native AArch64 code generation.

## Features

- **Pest-based parser** with preprocessor support (`use` directives)
- **SSA-style intermediate representation** compatible with LLVM IR
- **Native AArch64 backend** with register allocation
- **Cross-platform testing** via Docker on macOS

## Quick Start

Build the compiler:

```bash
cargo build --release
```

Compile a TeaLang program to LLVM IR:

```bash
cargo run -- tests/dfs/dfs.tea
```

Compile to AArch64 assembly:

```bash
cargo run -- tests/dfs/dfs.tea -d s -o dfs.s
```

## Usage

```
teac [OPTIONS] <FILE>

Arguments:
  <FILE>  Input file (.tea source)

Options:
  -d <MODE>        Dump mode: ast, ir (default), or s (assembly)
  -o <FILE>        Output file (default: input with .ll or .s extension)
  -h, --help       Print help
```

### Examples

```bash
# Dump AST
cargo run -- program.tea -d ast

# Generate LLVM IR (default)
cargo run -- program.tea -o program.ll

# Generate AArch64 assembly
cargo run -- program.tea -d s -o program.s
```

## Project Structure

```
src/
├── ast/          # Abstract Syntax Tree definitions
├── ir/           # Intermediate Representation & code generation
│   └── gen/      # IR generation from AST
├── asm/          # Assembly backends
│   ├── aarch64/  # AArch64 code generation & register allocation
│   └── common/   # Shared backend utilities
├── parser.rs     # Pest-based parser implementation
├── main.rs       # CLI entry point
└── tealang.pest    # Grammar definition
```

## Testing

Run the full test suite:

```bash
cargo test
```

### Platform Requirements

| Platform | Requirements |
|----------|--------------|
| **AArch64 Linux** | Native — just `gcc` |
| **x86/x86_64 Linux** | Cross-compiler + QEMU: `sudo apt install gcc-aarch64-linux-gnu qemu-user` |
| **macOS** | Docker Desktop (uses ARM64 Linux containers) |

## Resources

- [Pest Parser Repository](https://github.com/pest-parser/pest)
- [Pest Book (Documentation)](https://pest.rs/book/)