# Preface

Most Rust books teach you how to write valid Rust. This book starts later: the code
compiles, the tests pass, and a reviewer still leaves a comment.

Rust has excellent introductions. *The Book*, *Rustlings*, *100 Exercises to Learn
Rust*, and others teach the language better than I could. I recommend them. But in
workshops and code reviews, I kept seeing the same gap. Once several implementations are
correct, how do you choose the one you want to maintain?

Why should this function take `&str` instead of `String`? Why does an error disappear
two layers down? Is that tuple carrying a domain concept that needs a name?

The answers come from experience with interfaces, callers, and edge cases. We often
call that experience *taste*. The word is vague, but the skill is concrete. You can
learn the questions behind it.

The usual advice is to read a lot of code and work with senior engineers. That works,
but it takes time and assumes your team has experienced Rust reviewers available. This
book tries to shorten that feedback loop.

## How it works

Each chapter begins with a small piece of working Rust, much like something you might
find in a real pull request. Before you see a possible solution, you'll ask:

- What contract do the tests establish?
- What does the function ask from its callers?
- Which cases are missing?
- What would a refactor improve, and what would it cost?

The possible solution stays hidden until you choose to open it. Treat it like a review
comment: consider it, challenge it, and decide whether it fits your context. A shorter
implementation can obscure an unfamiliar invariant. A stricter type can burden every
caller. An iterator can avoid an allocation while making debugging harder for the team
that owns the code.

Every solution is examined through the same six lenses: type-safety, readability, API
design, performance, flexibility, and error reporting. The scores make the reasoning
explicit. They do not turn review into arithmetic.

You'll also meet green tests that protect the wrong behaviour or miss the input that
matters. The edge-case sections follow those loose threads into production. The quizzes
ask you to choose between reasonable alternatives, and the concept index connects ideas
across chapters.

## Is this book for you?

You should already be comfortable reading basic Rust. If `?` is unfamiliar or you are
still learning why `String` and `&str` differ, start with *The Book* or *100 Exercises to
Learn Rust*, then come back. These chapters focus on design choices around the language
features.

I have opinions and will explain them. Your callers, team, or constraints may lead you
to a different answer. That disagreement is part of the exercise. A book cannot replace
a thoughtful review from someone who knows your system, but it can give you better
questions for that conversation.

## Workshop and licensing

I teach this material as a live workshop for teams. It uses the same exercises with more
time for discussion and disagreement. You can find it at <https://corrode.dev>.

The Rust code in this repository is MIT-licensed, so you can copy and adapt it. The
prose is all-rights-reserved by corrode, with permission to quote short passages for
review or teaching with attribution. The reasoning behind that split is in
[the book license](https://github.com/corrode/refactoring/blob/main/book/LICENSE.md).

Thanks to everyone who challenged an early workshop solution. Many of those objections
now appear in the trade-off sections.

Let's look at some Rust.
