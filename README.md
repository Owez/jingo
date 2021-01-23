# Jingo

[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fowez%2Fjingo%2Fbadge&style=for-the-badge)](https://actions-badge.atrox.dev/owez/jingo/goto)
![License](https://img.shields.io/github/license/owez/jingo?style=for-the-badge)

A lightweight, high-level language designed for rapid prototyping

## Syntax example

A small class-based program demonstrating the basic syntax of this language:

```none
--- Small test class, helping to describe some features of Jingo
class SomeClass;

--- Creates new [SomeClass] from `x` value
fun SomeClass::new(x) {
  this.x = x; -- make it an x, this is a comment!
}

--- Multiplies number on record with `y`
fun SomeClass.multiply(y) {
  return this.x;
}

my_class = SomeClass::new(3);
my_class.multiply(2); -- will be 3 * 2 = 6
```

## Installation

1. Clone this repository
2. Build using `cargo build --release`
3. Use the compiled binary at `/target/release/jingo-cli`

## Help

```none
Usage: jingo [OPTIONS]

A lightweight, high-level language designed for rapid prototyping

Options:
  run [FILE] — Compiles & runs a file
  compile [FILE] — Compiles a file
  help — Shows this help

Advanced options:
  scan [FILE] — Returns scanning stage only
```

## Under the hood

The main general ruling Jingo sticks to for its syntax are:

- Class-based OOP
- Dynamic
- Expression-centric; everything is an expression

As for compiler backends, this repository may count as both a transpiler and compiler; the Jingo syntax is parsed then transpiled to a simple [LISP](https://en.wikipedia.org/wiki/Lisp_(programming_language)) intermediate representation which is further compiled into a single static binary. This approach allows easier implementation of Jingo using a widely used and understood technology, as well as the future possibility of importing LISP-based code/modules.

Despite this, Jingo is not intended to be an abstract LISP flavour and considers it to be a suitable and extendible compiler backend, similar to other options like [LLVM](https://en.wikipedia.org/wiki/LLVM) or [cranelift](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift).
