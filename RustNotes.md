## Compiling and Build Rust Projects

- Create a new project using `cargo new [name]`
- Build the project with `cargo build`
- Build and run the project with `cargo run`
- Build project without creating an executable `cargo check`
    - Cargo stores it in the `target/debug` directory
- Build for release, optimized, using `cargo build --release`
    - Cargo stores it in the `target/release` directory
- When cloning a git repo, cd into directory and use `cargo build`
- Use `rustup` to update Rust
- Complie `*.rs` files using `rustc`

## Variables

- Variables are immutable by default.
- Add `mut` to make a varible mutable.
- References are immutable by defult, even if the variable is mutable.
- Use `&mut variable` instead of `&guess` to make references mutable.
- You can reuse variable names by *shodowing* them.
    - Redeclare the variable (`let`, `const`, etc.)
    - The *showdowing* variables only last within the scope ends.
    - The *showdowed* variables value presists but only accesable after the *shadowing* variable' scope ends.
- `mut` does not allow you to change the type of the variable.
- *showdowing* a variables does allow the type of the variable to be changed.

## Documentation
- `cargo doc --open` builds documentation provided by your dependencies.

## Types
| Length  | Signed | Unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |
