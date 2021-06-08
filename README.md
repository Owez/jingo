# Jingo

[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fowez%2Fjingo%2Fbadge&style=for-the-badge)](https://actions-badge.atrox.dev/owez/jingo/goto)
![License](https://img.shields.io/github/license/owez/jingo?style=for-the-badge)

A lightweight, high-level language designed for rapid prototyping

## Syntax example

A small class-based program demonstrating the basic syntax of this language:

```none
print("Hello, world!")

--- Small test class, helping to describe some features of Jingo
class SomeClass {
    --- Creates new [SomeClass] from `x` value
    fun new(x) {
        let self.x = x
    }

    --- Multiplies number on record with `y`
    fun multiply(self, y) {
        return self.x * y
    }
}

let multiplied = SomeClass.new(3).multiply(25)

match == multiplied + 10 {
    10 => print("Huh? 10?"),
    85 => print("This is the number!"),
    _ => print("Default case")
}
```

Jingo follows these rules for syntax:

- No semi-colons or forced tabbing
- Everything is an expression
- Match-orientated conditionals

## Installation

1. Clone this repository
2. Build using `cargo build --release`
3. Use the compiled binary at `/target/release/jingo-cli`

## Help

```none
Usage: jingo [OPTIONS]

A lightweight, high-level language designed for rapid prototyping

Options:
  run [FILE]    Compiles & runs a file
  build [FILE]  Compiles a file
  help          Shows this help

Advanced options:
  lex [FILE]    Show lexing output
  parse [FILE]  Show parsing output
```
