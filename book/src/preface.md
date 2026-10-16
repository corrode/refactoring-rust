# Preface

This book is about reviewing and refactoring working Rust code. When several
implementations are correct, how do you choose one to maintain?

Rust has excellent introductions. *The Book*, *Rustlings*, and *100 Exercises to
Learn Rust* teach ownership, syntax, and the standard library. This book assumes
you know the basics and want to practise design decisions: whether a function
should take `&str` instead of `String`, whether an error reaches the caller, or
whether a tuple needs a named type.

These decisions depend on experience, the code's requirements, and the people
who maintain it. The exercises give you practice asking review questions and
explaining your choices.

## How it works

Each chapter begins with a small piece of working Rust. Before you see a
possible solution, you'll ask:

- What contract do the tests establish?
- What does the function ask from its callers?
- Which cases are missing?
- What would a refactor improve, and what would it cost?

Outside the domain-modeling section, the first solution keeps the original API, including function signatures and caller-facing types.
That lets you improve the implementation without making callers change their code.
Where helpful, we then explore alternative APIs and discuss what callers gain or lose.
For example, borrowing an input or returning an iterator may be worth changing the interface, but we treat that as a separate decision.

In [Domain modeling](part-5-domain-modeling/README.md), designing the API is part of the exercise.
There, even the first solution can change types and signatures; we'll discuss how callers and tests need to adapt.

Then you can open the solution and compare. It is *a possible solution*, not
necessarily the best choice for your code. A shorter implementation can make
an invariant harder to understand. A stricter type can add work for callers.
An iterator can avoid an allocation while making debugging harder for the team.
Sometimes a loop is the better choice.

The original and solution variants are compared against the same six criteria: type-safety,
readability, API design, performance, flexibility, and error reporting. These
criteria help you explain a preference and discuss it with your team.

Some exercises have passing tests that protect the wrong behaviour or miss an
important input. The edge-case sections discuss those gaps and other production
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
