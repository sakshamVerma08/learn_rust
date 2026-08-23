# learn_rust 🦀

My workspace for learning Rust by working through **[The Rust Programming Language](https://doc.rust-lang.org/book/)** ("the Book"), chapter by chapter. Each folder is a standalone Cargo project holding the exercises and mini-projects tied to a specific chapter or concept.

## Structure

Each subdirectory is its own Cargo package (own `Cargo.toml`, `Cargo.lock`, and `src/`), so they can be built and run independently.

| Project | Concepts covered |
|---|---|
| [`hello_cargo`](./hello_cargo) | First steps with Cargo — project setup, build, and run |
| [`variables`](./variables) | Variables, mutability, shadowing, and data types (integers, floats, type sizes) |
| [`control_flow`](./control_flow) | `if`/`else`, loops, and small CLI programs (e.g. a simple interest calculator, temperature converters) that read and parse user input |
| [`functions`](./functions) | Function definitions, parameters, return values, and expressions vs. statements |
| [`guessing_game`](./guessing_game) | The Book's classic guessing-game project — `rand`, `match`, `Ordering`, and basic error handling with `Result` |

## Running a project

Since each folder is its own Cargo package, `cd` into it and use the usual Cargo commands:

```bash
cd variables
cargo run
```

Or, without changing directories, build/run a specific package with `--bin`/`-p`... actually for these single-binary crates, just run from within the folder:

```bash
cd guessing_game
cargo run --release
```

Other handy commands:

```bash
cargo check   # fast type-check without producing a binary
cargo build   # compile
cargo run     # compile and run
```

## Notes

- Progress currently spans early chapters of the Book: Cargo basics, data types, variables, control flow, and functions.
- Some files intentionally explore Rust quirks (explicit `as` casting, integer overflow, the unit type `()`, how a trailing semicolon turns an expression into a statement, etc.) as part of building intuition for how Rust differs from other languages.
- This repo is a learning log, not production code — some functions are unused/experimental and left in for reference.

## Resources

- [The Rust Programming Language (the Book)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
