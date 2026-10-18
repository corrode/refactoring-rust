# Welcome to *Refactoring Rust*

This book is about reviewing and refactoring working Rust code. 

I wrote it because while Rust has excellent introductions like *The Book*, *Rustlings*, and *100 Exercises to
Learn Rust* which teach ownership, syntax, and the standard library, there is less material on how to get beyond that.

This book assumes you know the basics and want to practise idiomatic Rust and how to improve Rust code. 
It's meant as worked examples for you to think about better code and to discuss trade-offs with your team.
There is always more than one way to approach a problem, so don't rush through the exercises.
Instead, take the time to try multiple approaches and learn from other people's code.

You won't learn much by browsing the chapters either.
Writing better APIs depends on exercise and gaining experience and you won't get that from reading alone.
You have to sit down with a cup of hot chocolate and mold the code with your bare hands.
Don't ask a machine to do it for you; attempt to write the first version yourself, even if it takes longer. 

## How it works

Each chapter begins with a small piece of working Rust.
Sit with the code for a bit and look for problems.

- Do you understand the logic?
- Is the code overly complex or verbose?
- Is the API clear and precise? 

In [Domain modeling](part-5-domain-modeling/README.md), designing the API is part of the exercise.
Before that, try to stick to the original function signature and types.

Where helpful, we'll explore alternative APIs and discuss their tradeoffs. 

Feel free to run the code as often as you like using the "play" button in the browser.
You can also edit the code locally, of course.

Exercises come with tests.

he edge-case sections discuss those gaps and other production
concerns. The concept index connects ideas across chapters.

## Is this book for you?

You should already be comfortable reading basic Rust. If `?` is unfamiliar or
you are still learning why `String` and `&str` differ, start with *The Book* or
*100 Exercises to Learn Rust*. These chapters focus on design choices around
the language features.

I explain the reasons for my choices. Your callers, team, or constraints may
lead you to a different answer. Compare those reasons with your own rather
than trying to match the solution. The book provides review practice, but it
cannot replace a review from someone who knows your system.

## Workshop and licensing

I teach this material as a live workshop for teams. It uses the same exercises
with more time for discussion. You can find it at <https://corrode.dev>.

The Rust code in this repository is MIT-licensed, so you can copy and adapt it.
The prose is all-rights-reserved by corrode, with permission to quote short
passages for review or teaching with attribution. The reasoning behind that
split is in
[the book license](https://github.com/corrode/refactoring-rust/blob/main/book/LICENSE.md).

Thanks to everyone who challenged an early workshop solution. Many of those
objections now appear in the trade-off sections.
