# The trade-off axes

A refactor can look cleaner and still make the code worse for its callers. It
can remove an allocation while hiding an error. It can prevent one invalid
state while burdening every ordinary call with a new type. If a review only
names the improvement, it hasn't finished the job.

Every chapter in this book therefore scores its canonical solution against the
same six axes, in the same order, with the same vocabulary. The fixed rubric
matters. If each example invents its own framing, every discussion can sound
deep while teaching nothing reusable. By chapter 15, you'll have applied these
questions fourteen times. The aim isn't to memorise our scores. It's to make the
questions part of how you review code.

## The six axes

### 1. Type-safety

> Does the invariant live in the type system, or do we rely on
> convention and runtime checks?

This is the classic Rust win, and the axis we'll reach for most often. A
function that takes a `NonZeroU16` can't receive zero through Rust's safe API.
Construction checks a runtime value once, then the type carries that guarantee to every
later call. A function that takes a `u16` and uses `assert!` can still be correct, but
its caller has to inspect the implementation to learn about the restriction. Only the
first version preserves the invariant in the type system.

That move has a cost. You need to invent or find a type for the invariant, and
callers need to know how to construct it. If the condition is cheap to check at
runtime and a new type would spread complexity through the API, a runtime check
and a good test can be the better design. The book uses both approaches and
explains the choice each time.

### 2. Readability

> Would a teammate reading this in six months understand it without
> context, and how much context, exactly?

"Readable" often means "familiar to me." That's too vague for a review. Picture
an engineer who joins the team a year from now, has read the standard library
docs once, and is debugging a production incident at 11pm. Can they understand
the function by itself? Do they need its tests? Or do they have to read three
modules before the code makes sense?

The answer depends on the team. Engineers who use iterator chains every day may
find `s.split_whitespace().filter_map(...).collect()` immediately clear. A team
that mostly writes loops may not. The axis still holds: a refactor changes how
much context someone needs, and the review should say whether that amount went
up or down.

Clippy's lints often make readability suggestions for a typical Rust
programmer. They're usually right. They're sometimes wrong, especially when a
recommended pipeline would be clearer as two statements. This book follows
Clippy by default and marks the places where it disagrees. See
[how to run a code review](how-to-run-a-code-review.md).

### 3. API design

> Does the signature communicate intent? Are illegal states
> representable? Can the caller misuse it accidentally?

API design overlaps with type-safety, but they ask different questions.
Type-safety asks whether the compiler can reject a wrong call. API design asks
whether the signature tells the caller what the function does, what it needs,
and what it returns. `fn process(input: String, flag: bool, n: usize, also:
bool) -> Vec<u8>` type-checks and still tells you very little.

This axis covers:

- **Naming.** Does the function name match what it does?
- **Parameter order.** Is the subject first? Are related parameters adjacent?
- **Ownership.** Does the function take what it needs and no more (`&str` over
  `String` when it only reads)?
- **Return type.** Does the return type carry the right amount of information?
  Is it `Option` when it should be `Result`? Is it `Vec<T>` when it should be
  `impl Iterator<Item = T>`?
- **Newtypes.** Do domain concepts have their own types, or are they smeared
  into primitives?

This axis tends to move more than any other across the book. A two-line
signature change can make the next six improvements possible.

### 4. Performance

> What are the allocations, copies, branches, and asymptotic costs,
> honestly?

We score performance even when it doesn't decide the refactor. Especially then.
Naming the cost every time makes the rare hot path easier to recognise. A team
that always waves away an allocation loses the habit of noticing when
allocations become the bottleneck.

Within this book, performance usually means:

- **Allocations.** Does this allocate when it doesn't have to?
- **Copies.** Does this clone where a borrow would do?
- **Iteration style.** Do we return an eager `Vec` or a lazy `impl Iterator`?
- **Asymptotic class.** Is this `O(n)` or `O(n²)` for collections of a
  realistic size?

We won't benchmark anything here. We'll reason the way a code review does: by
inspection, while distinguishing *this matters in a hot path* from *this is fine
for the call sites we know about*.

### 5. Flexibility

> How easy is this to extend, compose, or generalise when the
> requirements change?

This is the axis the canonical solution loses on most often. That's
intentional. `fn count_words(s: &str) -> usize` is rigid in a useful way:
it does one thing and is hard to misuse. Generalising it to `fn count<T, P:
Fn(&T) -> bool>(items: &[T], pred: P) -> usize` improves flexibility, but the broader
name, signature, and type parameters make the interface harder to understand and use.

Rust makes premature abstraction tempting. A `Strategy` trait with one
implementation, a generic parameter used with one type, or a trait object added
in case the backend changes later all ask today's code to pay for a hypothetical
future. Write the rigid version first. Generalise when a second concrete use
case arrives.

An honest score says when a solution gives up flexibility on purpose and why
that restriction is useful.

### 6. Error reporting

> When this fails, does the caller learn enough to act?

`Result<T, ()>`, `Result<T, Box<dyn Error>>`, and `Result<T, MyError>` all
compile. If `MyError` has fourteen variants and each carries the offending
input, the three APIs still tell callers very different things.

The right amount of information depends on the caller's next step. A parsing
library should distinguish invalid syntax at line 5 from an unexpected end of
file. A CLI using that parser needs enough detail to print a useful message. An
internal function with one call site, where failure can only be logged and
ignored, can carry less.

Error reporting deserves its own axis because it often degrades quietly. A
function starts with an informative error type, then an intermediate layer adds
`.map_err(|_| MyError::Generic)` because propagation is inconvenient. Repeat
that a few times and the caller can no longer act on the error. Scoring this
separately forces us to notice the loss.

## A worked example

Let's apply all six axes to a small refactor: a function that decides whether a
string means "yes." The example is deliberately small so the scoring stays in
focus.

The starter:

```rust,editable
pub fn is_yes(s: String) -> bool {
    if s == "y" {
        return true;
    }
    if s == "yes" {
        return true;
    }
    if s == "Y" {
        return true;
    }
    if s == "YES" {
        return true;
    }
    false
}
```

A possible refactor:

```rust,editable
pub fn is_yes(s: &str) -> bool {
    matches!(s.to_ascii_lowercase().as_str(), "y" | "yes")
}
```

Scores are relative to the starter. The vocabulary is **wins / loses / neutral /
N/A**, followed by a sentence that explains the judgement.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions accept any `&str` / `String`. Neither encodes the "yes-ness" of the input in a type. We could, with a `Confirmation` enum and a `FromStr` impl, but for a CLI flag check that's overkill. |
| Readability | wins | One line, one expression, no early returns. A reader sees the full set of accepted answers in one glance. |
| API design | wins | `&str` is the natural type for a read-only check; `String` forced the caller to allocate or `.clone()` when they had only a borrow. |
| Performance | loses | `to_ascii_lowercase` allocates a new `String` on every call. The starter does no allocation. For a CLI flag this is irrelevant; in a hot loop it would matter. |
| Flexibility | loses | The refactor commits to ASCII-only case folding. If we later need general Unicode-aware matching, such as accepting `"SÍ"` as `"sí"`, we'd need another design. The starter is easy to extend with another exact spelling. |
| Error reporting | N/A | Under this predicate's contract, every string has a `true` or `false` answer. There is no failure case to report. A parser that must distinguish "no" from "invalid input" would need another return type. |

Four details matter here:

1. **The refactor loses on two axes.** That's normal. If every refactor wins
   everywhere, the scoring is too generous.
2. **The losses aren't automatic deal-breakers.** For a CLI flag, the
   allocation and ASCII restriction are acceptable. In a parser's hot loop,
   with user input that may contain non-ASCII text, they aren't. That context
   calls for another design.
3. **The `N/A` is genuine.** This predicate has no failure case, so error reporting
   does not apply. Still, check every `N/A`. It can reveal a question the code should
   answer.
4. **Clippy didn't decide the score.** It would suggest `matches!` and `&str`,
   but it wouldn't warn about the allocation or the lost flexibility. The
   rubric catches what the lint cannot.

The next twenty-seven examples use the same method. The decisions get harder as
the code grows, but the questions don't change.

## On the limits of the rubric

Six axes are enough for a useful shared vocabulary without pretending to cover
every concern. Five missed important costs. Seven split concerns too finely. The
reference chapters, 01, 14, and 12, didn't expose another axis the rubric kept
avoiding, so the list stops here.

Some chapters still need an extra question. Compile time is real. Binary size
matters on embedded targets. Concurrency safety matters when a function lives
behind an `Arc`. We discuss these concerns where they affect the decision, but
scoring them in every chapter would dilute the signal.

> **CONCEPT**
>
> When the rubric can't settle a trade-off, ask: *what would change about the
> team's life if we merged this version instead of the other one?* If the honest
> answer is "nothing meaningful," pick the simpler version and move on.
