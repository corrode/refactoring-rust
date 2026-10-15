# Refactoring Rust

*Hands-on exercises for writing more idiomatic, maintainable Rust.*

![Sample exercise](./assets/exercise1.jpg)

With Rustlings and 100 exercises to learn Rust, there exist two excellent
resources for learning Rust.

What I found to be lacking was a more advanced course, which teaches Rust
idioms and patterns by means of refactoring exercises, for typical real-world
scenarios.

That's what "Refactoring Rust" is all about.

Each exercise asks you to take a small, working-but-awkward piece of code
and make it more idiomatic. There is typically more than one good answer.
The goal is not to be a definitive source of truth, but rather to give us
something concrete to talk about.

## The book

The companion mdBook is an unfinished draft. Twelve exercise chapters have prose;
the remaining sixteen still need writing. Every rendered page carries a draft notice.
See [`book/PLAN.md`](./book/PLAN.md) for the plan.

Open it locally with live reload (requires [mdBook](https://rust-lang.github.io/mdBook/)):

```sh
make book
```

Run `make book-check` to test the examples and snippets, then build the HTML.
Successful `book` workflow runs attach a `book-draft` artifact. Download and extract
it, then open `index.html` to review the rendered book. The workflow does not publish
a public site.

## Running an exercise

```sh
cargo test --example 01_starts_with_uppercase
```

## Overview

The examples are numbered roughly by complexity. We'll start with quick
warm-ups everyone can finish in a few minutes and gradually move into
larger structural refactors. Feel free to skip around: each example is
self-contained.

