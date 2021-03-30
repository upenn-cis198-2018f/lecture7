# Lecture 7: Functional Programming with Closures and Iterators

## Reminders

- Homework 2 was graded -- if you still have questions about your grade that you want me to look into, please let me know and direct me to the GitHub feedback thread!

- Homework 3 [here](https://github.com/upenn-cis198/homework3).
**Due delayed to Monday, April 5 (will announce on Piazza)**

- Remember to email **before** the deadline to use late days (either the 4 freebies or additional late days) -- so that we make sure to delay grading your assignment.

## Outline of lecture

### Closures (March 25)

Closures are functions with a captured set of variables to evaluate them.
Closures are very powerful and allow functional programming in Rust
using the `Fn` trait.
We will discuss:

- How to write closures;

- How to write functions taking functions as arguments;

- The `Fn` trait for function types; and

- The useful `impl Trait` syntax.

### Iterators (April 1)

The Iterator trait is a very powerful feature in Rust that lets
you define objects that can be iterated over in a for loop. We will discuss:

- How to use iterators to write concise, clean code in Rust;

- How to define iterators for your own data types, using the `Iterator` and `IntoIterator` traits.
