# Jingo

A lightweight, high-level language designed to be sleek and robust.

## Examples

Simple oop example:

```jingo
-!- This snippet shows how classes work in Jingo.

--- The breakfast class, providing the main breakfast implamentations found
--- inside of this snippet
class Breakfast;

--- Automatically gets passed `this`
fun Breakfast.new(food) {
	this.food = food;
}

--- Prints out food for breakfeast
fun Breakfast.print_food() {
	print(this.food);
}

fun main() {
	var my_breakfast = Breakfast.new("cool_food");

	my_breakfast.print_food(); -- Will print `Apples`
	my_breakfast.food = "Cherries"; -- Change `Apples` to `Cherries`
	my_breakfast.print_food(); -- Will now print `Cherries`
}
```

*All above where taken from the [`examples/` directory](https://github.com/scOwez/jingo/tree/master/examples) inside of [the repository](https://github.com/scOwez/jingo/).*

## File structure


In [the repository](https://github.com/scOwez/jingo/), you will find .jno/.jino examples inside of the `examples/` directory, the official CLI in the `jingo/` directory and the core library for Jingo in the `jingo-lib/` directory.
