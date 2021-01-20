# Jingo

[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fowez%2Fjingo%2Fbadge&style=for-the-badge)](https://actions-badge.atrox.dev/owez/jingo/goto)
![License](https://img.shields.io/github/license/owez/jingo?style=for-the-badge)

A lightweight, high-level language designed for rapid prototyping

## Guide

You may find a guide and specification to using Jingo or help with contributing to the compiler online [here](https://github.com/Owez/jingo/wiki), located as the GitHub wiki section.

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
