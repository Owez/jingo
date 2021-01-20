# Jingo

[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fowez%2Fjingo%2Fbadge&style=for-the-badge)](https://actions-badge.atrox.dev/owez/jingo/goto)
![License](https://img.shields.io/github/license/owez/jingo?style=for-the-badge)

A lightweight, high-level language designed for rapid prototyping

## Guide

Included in the compiler repository is the "The Jingo Book", a simple guide and specification to the Jingo language. You may find the book online [here](https://github.com/Owez/jingo/blob/master/book.md).

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
