# Preface

Rust has a problem the rest of the language ecosystem would love to
have: too many good introductions. *The Book* will teach you the syntax
and the borrow checker. *Rust by Example* will give you a working
snippet for almost anything you can name. *Rustlings* will drill you
on the corners of the standard library until they're muscle memory.
*100 Exercises to Learn Rust* will walk you through the language end to
end at a sensible pace. *Programming Rust* and *Rust for Rustaceans*
will fill in everything those leave out. Brown University's fork of
*The Book* will even quiz you on ownership with diagrams I genuinely
wish I'd had when I learned.

None of those books will teach you taste.

By *taste* I mean the thing a senior reviewer is doing when they look
at a function that compiles, that passes the tests, that does what it's
supposed to, and writes a comment anyway. The function takes a
`String` where a `&str` would do. The error is swallowed two layers
down. The match could be a `let-else`. The `Vec` should be an
iterator. The newtype that would make the whole module legible is
hidden inside a `(u64, u64)` tuple. None of these are bugs. Most of
them aren't even obvious until they're pointed out. All of them are
the difference between code that *works* and code a team is happy to
inherit.

The standard advice for acquiring taste is "read a lot of code and
work with people more senior than you." That's correct and useless. It
takes years. Most teams don't have a deep enough bench of senior Rust
engineers to make it practical. And the people who could teach it
typically do so one PR at a time, which doesn't scale and doesn't
transfer when they leave.

This book is the workaround. Each chapter takes a short,
working-but-awkward piece of Rust (the kind of code you'd actually
see in a PR) and walks through the conversation a thoughtful reviewer
would have with the author. *What would you ask first? What would you
suggest? What would you push back on? What does each refactor cost?
What does the canonical solution still lose on?* By the end, the goal
isn't that you remember our answers. It's that you've internalised the
questions and can start asking them on your own team.

## What makes this book different

A few specific commitments, called out so you can hold the book to
them:

**Honest trade-offs.** Every chapter scores the canonical solution
against the same six axes (type-safety, readability, API design,
performance, flexibility, error reporting) and is explicit when the
solution *loses* on an axis. There is no global optimum. A book that
pretends otherwise is selling you a religion.

**Review over recitation.** The questions a chapter poses come
before the answers. We want you to spot the smells yourself. The
chapter's job is to confirm what you noticed, surface what you didn't,
and give you the vocabulary to talk about both with your team.

**Edge cases you'd hit in production.** Most exercise books stop at
"and now it compiles." We stop at "and now it survives the input you
weren't expecting." When a chapter ignores an edge case to keep the
solution focused, we say so out loud in an *Edge cases* callout.

**A concept index.** Every named idea (*newtype*, *let-else*,
*parse, don't validate*, *make illegal states unrepresentable*) gets
an entry in the back-matter cheatsheet, linked from every chapter that
touches it. The book is a graph, not a list. You can pull on any
thread and follow it across the whole text.

**Quizzes that test judgement.** Brown's fork popularised inline
quizzes. We borrow the idea and change the questions. Ours don't ask
*what does this print*. They ask *which of these signatures best
expresses the invariant*, or *which axis does this refactor improve at
the cost of which other axis*. The point is to rehearse the
review-style thinking, not the syntax.

## What this book is not

It is not a Rust introduction. If you don't know what `?` does or
why `&str` and `String` are different, start with *The Book* (or, for
that matter, *100 Exercises*) and come back. The exercises here assume
you can read the starter code without needing the language explained.

It is not a style guide. We have opinions, and we'll defend them, but
we won't pretend they're laws. Where we disagree with Clippy, or with
ourselves, we say so explicitly.

It is not exhaustive. There are twenty-seven exercises. They cover the
patterns I see come up most often in real Rust code reviews; they do
not cover everything. The patterns that *aren't* here are not in the
book because including them would have meant either padding or
trespassing on terrain other books already cover well.

It is not a substitute for working with senior engineers. It's a
substitute for *not* working with them.

## The workshop

I run this material as a live workshop for teams. The book is the
self-study path. The workshop is the same content with the discussion
turned all the way up: three to five hours of arguing about
trade-offs in a room, which is where most of the learning actually
happens. If your team would benefit, the booking form lives at
<https://corrode.dev>.

## A note on licensing

The Rust code in this repository (every starter, every solution,
every snippet quoted in this book) is MIT-licensed. Copy any of it
freely. The prose of the book is not. It's all-rights-reserved, ©
corrode, with permission to quote short passages for review or
teaching with attribution. The split is unusual; the reasoning is in
[book/LICENSE.md](https://github.com/corrode/refactoring/blob/main/book/LICENSE.md).
TL;DR: code is for using, prose is for reading.

## Thanks

To everyone who attended an early workshop and pushed back on the
canonical solutions hard enough to improve them: you'll find many of
your dissents reflected in the *Trade-offs* sections. To the broader
Rust community for being the rare technical community where "I
disagree, here's why" is treated as a contribution rather than a
threat. And to the maintainers of mdBook, Clippy, and rust-analyzer,
without whom this book would either not exist or would have to be
considerably more apologetic.

Enough preamble. Let's look at some Rust.
