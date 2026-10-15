# Preface

Most Rust books teach you how to write valid Rust. This book starts later: the code
compiles, the tests pass, and a reviewer still leaves a comment.

Rust has excellent introductions. *The Book*, *Rustlings*, *100 Exercises to Learn
Rust*, and others teach the language better than I could. Use them when you want to
learn ownership, syntax, or the standard library. This book is about the question that
comes next: once several implementations are correct, how do you choose the one you
want to maintain?

Why should this function take `&str` instead of `String`? Why does an error disappear
two layers down? Is that tuple carrying a domain concept that needs a name?

We often call the instinct behind these questions *taste*. That makes it sound more
mysterious than it is. A large part of taste is accumulated evidence: interfaces that
aged well, shortcuts that confused callers, and edge cases that returned as bugs. You
cannot compress years of context into twenty-seven exercises. You can make the questions
visible and practise asking them.

That is the modest promise of this book.

## How it works

Each chapter begins with a small piece of working Rust, much like something you might
find in a real pull request. Before you see a possible solution, you'll ask:

- What contract do the tests establish?
- What does the function ask from its callers?
- Which cases are missing?
- What would a refactor improve, and what would it cost?

Then you can open the solution and compare. I call it *a possible solution* for a
reason. A shorter implementation can obscure an unfamiliar invariant. A stricter type
can burden every caller. An iterator can avoid an allocation while making debugging
harder for the team that owns the code. Sometimes the plain loop is the right answer.
You don't get points for using the most combinators.

Every solution is examined through the same six lenses: type-safety, readability, API
design, performance, flexibility, and error reporting. A scorecard cannot review code
for you. It can force a vague preference into words, which gives your team something to
discuss.

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
[the book license](https://github.com/corrode/refactoring-rust/blob/main/book/LICENSE.md).

Thanks to everyone who challenged an early workshop solution. Many of those objections
now appear in the trade-off sections.

Start with the first function. Decide what you would say in review before you open the
answer.
