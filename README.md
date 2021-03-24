# Lecture 7: Iterators and Closures

## Reminders

- Homework 3 due on April 1: [here](https://github.com/upenn-cis198/homework3)

- Homework 2 was graded -- if you still have questions about your grade that you want me to look into, please let me know and direct me to the GitHub feedback thread!

- Remember to email **before** the deadline to use late days (either the 4 freebies or additional late days) -- so that we make sure to delay grading your assignment.

## Outline of lecture

**Closures:**
Closures are functions with a captured set of variables to evaluate them.
Closures are very powerful and allow functional programming in Rust
using the `Fn` trait.
We will discuss:

- How to write functions taking functions as arguments; and

- The `Fn` trait for function types, and the useful `impl Trait` syntax.

**Iterators:**
The Iterator trait is a very powerful feature in Rust that lets
you define objects that can be iterated over in a for loop. We will discuss:

- How to use iterators to write concise, clean code in Rust;

- Why you would want to use iterators for your own data and types; and

- How to define your own iterator implementing the `Iterator` trait.
