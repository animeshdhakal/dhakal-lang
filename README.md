# Dhakal Lang
By [animeshdhakal](https://github.com/animeshdhakal) (theanimeshdhakal@gmail.com)

Dhakal Lang is a lightweight, hobbyist-focused programming language designed to be simple, clean, and fun to explore. It’s a project built from the ground up to understand how programming languages work, offering a straightforward syntax that feels familiar if you've ever played with C or other scripting languages.

## Why Dhakal Lang?
Building a language should be accessible. Dhakal Lang prioritizes clarity, keeping the barrier to entry low for anyone interested in language design, interpreter implementation, or just experimenting with code. It doesn't aim to be the fastest or the most feature-rich—it aims to be a tool for learning and creativity.

## Key Features

*   **Clean Declarations:** Use the `val` keyword for assigning variables.
*   **First-Class Functions:** Define logic easily with the `func` construct.
*   **Control Flow:** Handle logic with standard `if` statements and `for` loops.
*   **Simple I/O:** Built-in `print` support for quick debugging and interaction.

## Syntax Gallery

### Functions & Recursion
Dhakal Lang makes defining recursive functions intuitive:

```rust
val n = 3

func fib(n) {
	if (n == 0) {
		return 0;
	}

	if (n == 1) {
		return 1;
	}

	return fib(n-1) + fib(n-2);
}

val value = fib(n);
print(value)
```

### Iteration
Write clean loops to handle repetitive tasks:

```rust
for(val i = 0; i < 10; val i = i + 1) {
	print("Hello: ", i);
}
```

## Getting Started

1.  **Clone the repository.**
2.  **Build:** Run `cargo build` in the project root to compile the interpreter.
3.  **Run:** Execute your `.dkl` files using the generated binary.
