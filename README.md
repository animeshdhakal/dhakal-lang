# Dhakal Lang

By [animeshdhakal](https://github.com/animeshdhakal) (theanimeshdhakal@gmail.com)

Dhakal Lang is a lightweight, hobbyist programming language built in Rust. It's designed to be simple, clean, and approachable for anyone curious about how programming languages work under the hood.

## Why Dhakal Lang?

Building a language should be accessible. Dhakal Lang prioritizes clarity over complexity, keeping the barrier to entry low for anyone interested in language design, interpreter implementation, or just experimenting with code. It isn't trying to be the fastest or the most feature-rich. It's a tool for learning and creativity.

## Key Features

- **Clean Declarations:** Use the `val` keyword for assigning variables.
- **First-Class Functions:** Define logic easily with the `func` construct.
- **Control Flow:** Handle branching with `if` statements and looping with `for`.
- **Arrays:** Work with ordered collections out of the box.
- **Comments:** Document your code with inline comments.
- **Simple I/O:** Built-in `print` support for quick debugging and interaction.

## Syntax Gallery

### Functions and Recursion

Dhakal Lang makes defining recursive functions intuitive:

```rust
val n = 10

func fib(n) {
    if (n == 0) {
        return 0;
    }

    if (n == 1) {
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
}

val value = fib(n);
print(value)
```

### Iteration

Write clean loops for repetitive tasks:

```rust
for (val i = 0; i < 10; val i = i + 1) {
    print("Hello: ", i);
}
```

## Getting Started

### Prerequisites

- Rust toolchain (1.70 or newer recommended)
- Cargo

### Build

```bash
git clone https://github.com/animeshdhakal/dhakal-lang.git
cd dhakal-lang
cargo build --release
```

### Run

Execute a `.dkl` file with the compiled binary:

```bash
./target/release/dhakal-lang path/to/script.dkl
```

## Project Status

Dhakal Lang is an active work in progress. Expect rough edges, breaking changes, and plenty of room to contribute ideas.

## License

This project is open for learning and experimentation. See the repository for details.

---

[animeshdhakal](https://github.com/animeshdhakal)
