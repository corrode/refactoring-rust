# Refactoring Rust

This book is about reviewing and refactoring working Rust code.

I wrote it because while Rust has excellent introductions like *The Book*, *Rustlings*, and *100 Exercises to Learn Rust* which teach ownership, syntax, and the standard library, there is less material on how to get beyond that.

This book assumes you can read basic Rust, including `?` and the difference between `String` and `&str`.
It's meant as worked examples for you to think about better code and to discuss trade-offs with your team.
There is always more than one way to approach a problem, so don't rush through the exercises.
Instead, take the time to try multiple approaches and learn from other people's code.

You won't learn much by browsing the chapters either.
Writing better APIs takes practice, and you won't get that from reading alone.
You have to sit down with a cup of hot chocolate and mold the code with your bare hands.
Don't ask a machine to do it for you; attempt to write the first version yourself, even if it takes longer.

I explain the reasons for my choices.
Your callers, team, or constraints may lead you to a different answer.
Compare those reasons with your own rather than trying to match the solution.
The book provides review practice, but it cannot replace a review from someone who knows your system.

Start with [the first exercise, `is_yes`](part-1-warmups/01-is-yes.md).
Try your own refactor before reading the solution.

[How to use this book](appendix/how-to-use.md) · [Errata and reuse](appendix/contributing.md)
