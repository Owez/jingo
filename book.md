# The Jingo Book

A guide and specification for the Jingo language, kept updated by the community.

## Overview

Here's a simple example of a functioning program using Jingo, with some of the basic syntax of this language used. It uses the [Fibonacci numbers](https://en.wikipedia.org/wiki/Fibonacci_number) with a spin 👐

```none
class FibNum;

fun FibNum.new(num) {
    this.num = num;
}

fun FubNum.calc(this, left, right) {
    this.num = left.num + right.num;
}

fun main() {
    var mut left = FibNum.new(0);
    var mut right = FibNum.new(1);

    var calc_to = 100;

    for _ in 0..calc_to {
        var tmp = FibNum.calc(left, right);

        left = right;
        right = tmp;
    }
}
```

As you may see, Jingo is a class-based (i.e. OOP) language. The syntaxical design is intended to be similar in parts to [Rust](https://en.wikipedia.org/wiki/Rust_(programming_language)) with some high-level design stemming from [Python](https://en.wikipedia.org/wiki/Python_(programming_language)).

## Specification

The following is a short n' sween specification for the syntaxical components of Jingo; this is intended to be more of an API reference for contributers for the compiler or interested minds.

### Commenting

Comments can be made using the `-- <text>` decleration for normal comments or `--- <text>` for documentation strings. Both of these types of comments should ideally be formatted in [markdown](https://en.wikipedia.org/wiki/Markdown) for furture support in documentation generators for Jingo, and general ease of usability in the current development landscape.

Documentation strings may only be used on data structures such as classes or functions or a compiler error will be raised.

### Variable declerations

Jingo uses constant variable declerations by default, i.e:

```none
var x = 10;

x = 20; -- won't compile
```

Allowing small compiler optimisationsd and less pressure on edge-cases for developers when it comes to references. This is taken from the [relevant]() Rust syntax.

Mutables are still an option of course and can be used with the `mut` token like so:

```none
var mut x = 10;

x = 20; -- will compile
```
