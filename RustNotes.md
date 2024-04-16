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

## Integer Types
| Length  | Signed | Unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

- Rust defaults to i32

## Floating-Point Types
| Length | Name |
|--------|------|
| 32-bit | f32  |
| 64-bit | f64  |

- Rust defaults to f64

## Tuples
- `let tup: (i32, f64, u8) = (500, 6.4, 1);`
- Deconstructing a tuple
    - `let (x, y, z) = tup;`
- Acessing a tuple directly
    - `let x = tup.0;`
    - `let y = tup.1;`
    - `let z = tup.2;`

## Arrays
- Simple
    - `let arr = [1,2,3];`
    - `let arr: [i32, 3] = [1, 2, 3]`
        - specifiy type
    - `let arr = [3; 5];`
        - initializes array of size five with all 3's

## Functions
- By convention function in Rust use `snake_case` format for names
- Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope.
- You must specifiy the type in parameters `fn func(x: i32){};`
- Return values are specified with the `->`
    - `fn five -> i32 { 5 }
- The value returned is the final expression in the block
- You can specifiy return values with `return`
- Convention is to return implicitly

## Statements and Expressions
- You can create a scope block with `{ //new scope }`
- The last line in a scope block does not end with a `;` otherwise it becomes a statement
- Statements do not return a value
- You can set a variable to a scope block which creates a marco.

## Loop
- Rust has three loops: `loop`, `while`, `for`.
- You can assign lables to loops `lable_name: loop {}`
- You can `break` and `continue` specific loops by using lables `break 'lable_name;`
- while loops work like all the other while loops
- for loops look like python `for i in a {};`
- You can use a range  too `for i in (1..4) {};`
- You can reverse the loop too `for i in (1..4).rev() {};`

## Ownership





































