# The trade-off axes

Every chapter in this book ends with a *Trade-offs* section that scores
the canonical solution against the same six axes, in the same order,
using the same vocabulary. This chapter defines them once.

Why fix the axes? Because the alternative is what most engineering
writing does: invent a fresh framing for each example. That makes every
discussion *sound* deep and makes the book as a whole *teach* nothing.
A reader can't carry a vocabulary they've never seen twice. With fixed
axes, by the time you reach chapter 14 you've scored thirteen solutions
against the same rubric, and the rubric starts to live in your head.
That's the goal: not for you to memorise our scores, but for you to
internalise the *questions*.

Here they are.

## The six axes

### 1. Type-safety

> Does the invariant live in the type system, or do we rely on
> convention and runtime checks?

The classic Rust win, and the one we'll reach for most often. A
function that takes a `NonZeroU16` cannot be called with zero. The
checker rejects that program before it runs. A function that takes a
`u16` and `assert!`s it isn't zero cannot make that promise; the
caller has to read the body to know. Both can be correct. Only one
shifts the cost of correctness to compile time.

Type-safety is not free. Pushing an invariant into the type system
means inventing (or finding) a type to carry it, which means the
caller now has to know about that type. Sometimes the right answer is
to keep the runtime check and write a good test. We'll see both in
the book, and we'll be explicit about why each time.

### 2. Readability

> Would a teammate reading this in six months understand it without
> context, and how much context, exactly?

Readability is the easiest axis to abuse, because "readable" usually
means "I can read it." A clearer formulation: imagine the engineer who
joins the team a year from now, has read the standard library docs
once, and is debugging a production incident at 11pm. Would they
understand this function from the function alone? From the function
plus its tests? Only after reading three modules of context?

There is no universal answer. A team that writes a lot of iterator
chains will find `s.split_whitespace().filter_map(...).collect()`
trivially readable; a team that mostly writes loops will not. The
axis is real either way: every refactor either reduces or increases
the amount of context needed to make sense of the code, and we should
name which.

A note on Clippy: Clippy's lints are mostly readability suggestions
optimised for the typical Rust programmer. They are usually right.
They are sometimes wrong, particularly when they recommend chaining a
pipeline that would read more clearly as two statements. The book
follows Clippy by default and disagrees with it explicitly when it
does. See [how to run a code review](how-to-run-a-code-review.md).

### 3. API design

> Does the signature communicate intent? Are illegal states
> representable? Can the caller misuse it accidentally?

API design and type-safety overlap but aren't the same. Type-safety
asks *can the compiler stop a wrong call.* API design asks *does the
caller understand, from the signature alone, what the function does,
what it needs, and what it returns.* A function can be perfectly
type-safe and still have a terrible API: `fn process(input: String,
flag: bool, n: usize, also: bool) -> Vec<u8>` type-checks fine and
tells you nothing.

Concretely, API design covers:

- **Naming.** Does the function name match what it does?
- **Parameter order.** Is the subject first? Are related parameters
  adjacent?
- **Ownership.** Does the function take what it needs and no more
  (`&str` over `String` when only reading)?
- **Return type.** Does the return type carry the right amount of
  information? Is it `Option` when it should be `Result`? Is it
  `Vec<T>` when it should be `impl Iterator<Item = T>`?
- **Newtypes.** Are domain concepts wrapped in their own types, or
  smeared into primitives?

This axis tends to move the most across the book. A two-line refactor
of a signature often unlocks the next six chapters of improvement.

### 4. Performance

> What are the allocations, copies, branches, and asymptotic costs,
> honestly?

We score performance honestly even when it doesn't matter. *Especially*
when it doesn't matter. The reason: a discipline of always naming the
performance trade-off makes the rare case where it *does* matter much
easier to recognise. A team that hand-waves performance ("the
allocator is fast, don't worry about it") loses the ability to notice
when the allocator suddenly becomes the bottleneck.

In practice "performance" for the book's scope usually means:

- **Allocations.** Does this allocate where it didn't have to?
- **Copies.** Does this clone where a borrow would have done?
- **Iteration style.** Eager `Vec` vs. lazy `impl Iterator`?
- **Asymptotic class.** `O(n)` vs. `O(n²)` for collections of
  realistic size?

We will not benchmark anything in this book. We will reason about
performance the way a code review does: by inspection, with a clear
distinction between *this matters in a hot path* and *this is fine for
the call sites we know about*.

### 5. Flexibility

> How easy is this to extend, compose, or generalise when the
> requirements change?

Flexibility is the axis the canonical solution most often loses on.
That's intentional. A signature like `fn count_words(s: &str) ->
usize` is rigid in a specific, useful way: it does one thing well and
can't be misused. Generalising it to `fn count<T, P: Fn(&T) -> bool>
(items: &[T], pred: P) -> usize` makes it more flexible at the cost
of every other axis.

Premature flexibility (the abstract `Strategy` trait with one
implementation, the generic parameter that's only ever instantiated
with one type, the trait object that exists "in case we want to swap
the backend later") is one of the most common ways Rust code goes
bad. The book is going to suggest you write the rigid version first
and reach for flexibility only when a second concrete use case
exists.

Honest scoring on this axis means saying when a solution *gives up*
flexibility deliberately, and why.

### 6. Error reporting

> When this fails, does the caller learn enough to act?

A function that returns `Result<T, ()>` compiles. So does one that
returns `Result<T, Box<dyn Error>>`. So does one that returns
`Result<T, MyError>` where `MyError` is a 14-variant enum, each
variant carrying the offending input. They are not the same code.

The right amount of error information depends on what the caller will
do with it. A library exposing a parser should distinguish "invalid
syntax at line 5" from "unexpected EOF". A CLI consuming that parser
needs to produce a useful message. A function called only from one
internal site, where the only sane response to failure is to log and
move on, can get away with much less.

Error reporting is its own axis (not just a subset of API design)
because it's the axis that *quietly* degrades. A function that started
its life returning a perfectly informative error type often acquires a
`.map_err(|_| MyError::Generic)` somewhere on its journey because the
intermediate function couldn't be bothered to propagate. The damage
compounds. Calling it out as its own axis forces us to look.

## A worked example

Let's score one refactor against all six axes, to make the rubric
concrete. We'll use a slightly silly example so we can focus on the
method rather than the code: a function that returns whether a string
is a "yes" answer.

The starter:

```rust
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

```rust
pub fn is_yes(s: &str) -> bool {
    matches!(s.to_ascii_lowercase().as_str(), "y" | "yes")
}
```

Now the rubric. We score the refactor *relative to the starter*. The
shorthand vocabulary is **wins / loses / neutral**, plus a sentence.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions accept any `&str` / `String`. Neither encodes the "yes-ness" of the input in a type. We could, with a `Confirmation` enum and a `FromStr` impl, but for a CLI flag check that's overkill. |
| Readability | wins | One line, one expression, no early returns. A reader sees the full set of accepted answers in one glance. |
| API design | wins | `&str` is the natural type for a read-only check; `String` forced the caller to allocate or `.clone()` when they had only a borrow. |
| Performance | loses | `to_ascii_lowercase` allocates a new `String` on every call. The starter does no allocation. For a CLI flag this is irrelevant; in a hot loop it would matter. |
| Flexibility | loses | The refactor commits to ASCII-only case folding. If we later need to accept "Sí" or "はい", we'd have to redesign. The starter, by being explicit per case, is trivial to extend by adding a line. |
| Error reporting | N/A | Neither version reports errors. The function returns `bool`. We should ask whether the caller would benefit from distinguishing "explicitly no" from "didn't parse," but that's a follow-up, not a trade-off of this refactor. |

A few things worth noticing about this score:

1. **The refactor loses on two axes.** That's normal. There is no
   refactor that wins on every axis; if you find one, you're scoring
   too generously.
2. **The losses are honest costs, not deal-breakers.** Whether the
   refactor is worth it depends on the call site. For a CLI flag,
   absolutely. For a parser hot loop on user input that may contain
   non-ASCII, no. We'd want a different design.
3. **The `N/A` is genuine.** The starter and the refactor have the
   same error-reporting story (none), so there's nothing to score.
   When you find yourself writing N/A, double-check that the axis
   really doesn't apply. Sometimes it points at a question you
   *should* be asking.
4. **No axis was decided by Clippy.** Clippy would suggest the
   refactor (specifically, the `matches!` and the `&str`). It would
   not warn about the allocation, and it would not warn about the
   loss of flexibility. The trade-off framework picks up what Clippy
   doesn't.

The chapters that follow apply this same rubric to twenty-seven real
examples. The scores get more interesting as the examples get
larger, but the method is exactly this.

## On the limits of the rubric

Six axes is a deliberate choice. Five felt like it was missing
something; seven felt like it was inventing distinctions to seem
thorough. If the reference trio of chapters (01, 14, 19) had exposed
a sixth axis the rubric kept dancing around, we'd have added it. They
didn't, so this is the list.

You will, occasionally, want a seventh axis for a specific chapter.
Compile time is a real one. Binary size matters for embedded
targets. Concurrency safety matters when the function lives behind an
`Arc`. We mention these in chapters where they bite, but we don't
score them, because scoring them everywhere would dilute the
signal.

> **CONCEPT**
>
> When in doubt, ask the trade-off the rubric *can't* answer: *what
> would change about the team's life if we merged this version
> instead of the other one?* If the answer is "nothing meaningful,"
> the trade-off probably wasn't real and we should pick whichever is
> simpler.
