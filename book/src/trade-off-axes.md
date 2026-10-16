# The trade-off axes

A refactor can improve one aspect of code and make another worse. It may
remove an allocation but hide an error, or prevent an invalid state but add
work for callers. Review both the benefits and the costs.

Each exercise compares the original and solution variants against six criteria,
called axes, in the same order. Each variant has its own column, with a score and
a short reason for each criterion. Use them to compare implementations and explain
your decisions. The scores are judgements to examine, not answers to memorise.

## The six axes

### 1. Type-safety

> Does the type system enforce the invariant, or does the code rely on
> convention and runtime checks?

A function that takes a `NonZeroU16` can't receive zero through Rust's safe API.
Construction checks a runtime value once, then the type carries that guarantee to every
later call. A function that takes a `u16` and uses `assert!` can still be correct, but
its caller has to inspect the implementation to learn about the restriction. Only the
first version preserves the invariant in the type system.

This has a cost. You need to define or find a type for the invariant, and
callers need to know how to construct it. If the condition is cheap to check at
runtime and a new type would spread complexity through the API, a runtime check
and a good test can be the better design. The book uses both approaches and
explains the choice each time.

### 2. Readability

> Can a teammate understand this code without knowing why it was written?
> What other context do they need?

Familiarity affects readability. Be specific about what makes the code easier
or harder to understand. Can someone follow the function on its own, or do
they need to read its tests and several related modules?

The answer depends on the team. Engineers who use iterator chains every day may
find `s.split_whitespace().filter_map(...).collect()` immediately clear. A team
that mostly writes loops may not. The axis still holds: a refactor changes how
much context someone needs, and the review should say whether that amount went
up or down.

Clippy's readability suggestions are useful, but a recommended iterator chain
can be clearer as two statements. This book follows
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
- **Newtypes.** Do domain concepts have their own types, or do they use primitive
  types that don't distinguish them?

### 4. Performance

> What allocations, copies, branches, and asymptotic costs does this code have?

Assess performance even when it doesn't decide the refactor. An allocation
that is acceptable in an occasional call may be costly in a hot path.

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

A narrow interface can be useful. `fn count_words(s: &str) -> usize`
it does one thing and is hard to misuse. Generalising it to `fn count<T, P:
Fn(&T) -> bool>(items: &[T], pred: P) -> usize` improves flexibility, but the broader
name, signature, and type parameters make the interface harder to understand and use.

A `Strategy` trait with one implementation, a generic parameter used with one
type, or a trait object added for a possible backend change can add complexity
without a current need. Start with the specific version and generalise when
another concrete use case justifies it.

If a solution reduces flexibility, explain why that restriction is useful.

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

Assess error reporting separately because a refactor can lose information. A
function starts with an informative error type, then an intermediate layer adds
`.map_err(|_| MyError::Generic)` because propagation is inconvenient. The caller may then lack the information it needs to handle the error.

## A worked example

This function checks whether a string means "yes." Compare the starter and
refactor using all six axes.

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

Compared with the original: **+** advantage, **=** comparable, **−** disadvantage,
**N/A** not applicable. Readability depends on familiarity.

| Criterion | Original: owned input, exact comparisons | Borrowed input + ASCII lowercase |
|---|---|---|
| Type-safety | Any string; boolean result | = No new type-level invariant |
| Readability | Four separate branches | + One expression, if `matches!` familiar |
| API design | Consumes `String` | + Borrows `&str` |
| Performance | No internal allocation | − Allocates lowercase copy |
| Flexibility | Can add arbitrary exact spellings | − Applies ASCII folding to every input |
| Error reporting | Every input yields a boolean | N/A |

The borrowed parameter removes the need to give up ownership, but lowercasing adds
an allocation inside the function. Whether this costs more overall depends on whether
the caller would otherwise need to clone the input.

There is also a behavior change that the scores do not capture: the refactor accepts
mixed-case inputs such as `"Yes"`, while the original does not. Confirm that this is
intended before treating it as a replacement. Neither version reports invalid input
separately from a negative answer.

The exercises use the same questions for larger changes.

## On the limits of the rubric

These six axes provide a common set of questions, but they don't cover every
concern. Compile time, binary size on embedded targets, and concurrency safety
may also affect a decision. Discuss these when they matter rather than adding
a score to every chapter.

> **CONCEPT**
>
> If the scores don't resolve a choice, compare the practical consequences for
> callers and maintainers. If there is no meaningful difference, choose the
> simpler version.
