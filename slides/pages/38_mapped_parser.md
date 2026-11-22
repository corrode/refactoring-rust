---
layout: default
zoom: 0.9
---

# 38 · `mapped_parser`

```rust
use std::marker::PhantomData;

pub trait Parser<I, O, E> {
    fn parse(&mut self, input: I) -> Result<O, E>;

    fn map<F, U>(self, f: F) -> Map<Self, F, O>
    where
        Self: Sized,
        F: FnMut(O) -> U,
    {
        Map { parser: self, f, _output: PhantomData }
    }
}

pub struct Map<P, F, O> {
    parser: P,
    f: F,
    _output: PhantomData<O>,
}
```

<div class="absolute bottom-4 left-12 right-12 text-xs opacity-70">
Review the trait, adapter, and helper together. API changes are allowed.<br>
Reduced adaptation of a real <a href="https://github.com/rust-bakery/nom">nom</a> refactor. Full source: <code>examples/38_mapped_parser.rs</code>.
</div>

<!--
Run cargo test --example 38_mapped_parser. All starter tests pass.
Keep the parsing trivial: Decimal uses str::parse to produce u32 or ParseIntError.
Preserve results, original errors, composition, and stateful FnMut transformations.
Keep the input generic. Do not reveal associated types before the discussion.
This is our reduced std-only adaptation, not a literal upstream excerpt.
-->

---
zoom: 0.9
---

# 38 · The adapter and a helper

```rust
impl<I, O, U, E, P, F> Parser<I, U, E> for Map<P, F, O>
where
    P: Parser<I, O, E>,
    F: FnMut(O) -> U,
{
    fn parse(&mut self, input: I) -> Result<U, E> {
        self.parser.parse(input).map(&mut self.f)
    }
}

pub fn parse_all<I, O, E, P>(
    mut parser: P,
    inputs: impl IntoIterator<Item = I>,
) -> Result<Vec<O>, E>
where
    P: Parser<I, O, E>,
{
    inputs.into_iter().map(|input| parser.parse(input)).collect()
}
```

<div class="absolute bottom-4 left-12 right-12 text-xs opacity-70">
<code>Decimal</code> implements <code>Parser&lt;&amp;str, u32, ParseIntError&gt;</code> using <code>input.parse()</code>.<br>
The helper collects successes in order and stops at the first error.
</div>

<!--
O is the inner parser's output, U the mapped output. Map stores P and F, not O.
map invokes its closure only on success and preserves its mutable state.
parse_all owns the parser, collects a Vec, and does not roll back earlier effects.
Empty input invokes neither parser nor mapper. No Clone bounds are necessary.
The full file and tests supply Decimal, imports, and usage examples.
-->

---

# 38 · Review

- Which types are independent choices?
- Can the same parser and input have several output types?
- Why does `Map` mention an output it never stores?
- What happens if you remove only that parameter?
- Where do the repeated parameters matter if ordinary calls infer them?

<!--
The starter deliberately permits Parser<&str, u32, ParseIntError> and
Parser<&str, bool, ParseBoolError> on the same Flexible type. This is not a bug.
Removing only the PhantomData field leaves an unused struct parameter; removing
O from Map also leaves O unconstrained in the impl (E0207). A bound mentioning
O is not enough: P can implement Parser<I, O, E> for several O values.
The marker is zero-sized and does not allocate, but has type-system effects.
Target the declarations and helpers, not a promised inference fix at every call.
-->

---
zoom: 0.9
---

# 38 · Possible solution: associated types

```rust
pub trait Parser<I> {
    type Output;
    type Error;

    fn parse(&mut self, input: I) -> Result<Self::Output, Self::Error>;

    fn map<F, U>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> U,
    {
        Map { parser: self, f }
    }
}

pub struct Map<P, F> {
    parser: P,
    f: F,
}
```

<div class="absolute bottom-4 left-12 right-12 text-sm">
Stop asking callers to supply a type that the implementation already determines.
</div>

<!--
For fixed Self and I, the implementation selects one Output and one Error.
Different input types can still have different associated types on the same Self.
This does not eliminate generics: I, P, F, and the mapped result still vary.
The relationship resembles Iterator::Item from exercise 23.
Keep Sized on the consuming map method, not the trait: both versions can parse
through a trait object. Dynamic dispatch is not a new benefit of this refactor.
-->

---
zoom: 0.85
---

# 38 · Use the parser's chosen types

```rust
impl<I, U, P, F> Parser<I> for Map<P, F>
where
    P: Parser<I>,
    F: FnMut(P::Output) -> U,
{
    type Output = U;
    type Error = P::Error;

    fn parse(&mut self, input: I) -> Result<Self::Output, Self::Error> {
        self.parser.parse(input).map(&mut self.f)
    }
}

pub fn parse_all<I, P>(
    mut parser: P,
    inputs: impl IntoIterator<Item = I>,
) -> Result<Vec<P::Output>, P::Error>
where
    P: Parser<I>,
{
    inputs.into_iter().map(|input| parser.parse(input)).collect()
}
```

<div class="absolute bottom-4 left-12 right-12 text-xs opacity-70">
<code>Decimal</code> now implements <code>Parser&lt;&amp;str&gt;</code>, with <code>Output = u32</code> and <code>Error = ParseIntError</code>.<br>
Full source: <code>solutions/38_mapped_parser.rs</code>. The parsing and collection algorithms are unchanged.
</div>

<!--
P and I determine the intermediate output, so Map needs no extra O or marker.
Its output is the mapping function's U; its error remains P::Error.
Result::map still moves successes, preserves errors, and calls the same FnMut.
parse_all still allocates a vector, preserves order, and stops at the first error.
Removing a zero-sized marker is not a runtime optimization.
-->

---

# 38 · Migrate the API, not the algorithm

```rust
// Before:
let mut parser: Map<Decimal, _, u32> = Decimal.map(|n| n.to_string());

// After:
let mut parser: Map<Decimal, _> = Decimal.map(|n| n.to_string());
assert_eq!(parser.parse("42"), Ok(String::from("42")));
```

- Move output/error choices into each trait implementation.
- General helpers use `P: Parser<I>` and `P::Output` / `P::Error`.
- A constrained helper can use `P: Parser<I, Output = u32>`.
- Update explicit wrapper, helper, and trait-object annotations.
- Keep stateful mapping and first-error tests, not only value assertions.

<!--
The before/after lines are alternatives, not a combined runnable implementation.
Most unannotated map calls need no source change. Explicit parse_all<I,O,E,P>
arguments become parse_all<I,P>. The helper still takes P by value, not &mut P.
Trait-object annotations become dyn Parser<&str, Output=u32, Error=ParseIntError>.
Both listings have API-specific attached tests in the book. Ten mirrored cases
check runtime behavior; the extra starter test records multiple output/error impls.
Integration fixtures reject conflicting output and error impls with E0119 and
positively check different inputs, helpers, and mapped parsers.
-->

---

# 38 · Fewer choices is a contract change

- One output/error pair per fixed parser and input type.
- Keep generics when callers genuinely need to choose those types.
- Output and error do not have to use the same design.
- No promised speed-up or universal inference improvement.
- Removing `PhantomData<O>` can also change `Send` / `Sync` bounds.

<div class="absolute bottom-4 left-12 right-12 text-xs opacity-70">
Source: <a href="https://github.com/rust-bakery/nom/pull/1626">nom PR #1626</a>, merged January 22, 2023.<br>
<a href="https://github.com/rust-bakery/nom/blob/4c6e45eded6e76a227f8069ec3f77f8a14b988c8/src/internal.rs">Before</a> · <a href="https://github.com/rust-bakery/nom/blob/bf8cddbdbdf768a2128b7817c4670da059351f9f/src/internal.rs">After</a> · <a href="https://doc.rust-lang.org/reference/items/associated-items.html#associated-types">Associated types</a>
</div>

<!--
The original trait was not unsafe. Removing alternative impls is an intentional
restriction, not an automatically behavior-preserving public API cleanup.
Use different parser types, or keep a parameter, for genuine independent choices.
A helper may retain E with P: Parser<I, Error=E> for readability; equality ties it
to the chosen parser rather than allowing a second Parser<I> impl for the same P.
The nom author kept named error parameters in many combinators for readability.
A winnow follow-up in the PR reports E0207: assigning an unconstrained E to the
Error associated type cannot determine E. Do not generalize this example to all APIs.
PhantomData<O> is zero-sized, but its owning form inherits auto-trait restrictions
from O; deleting it can relax them. Review variance and drop-checking roles too.
Our Result omits nom's remaining-input tracking, streaming, and error machinery.
Decimal, parse_all, and tests are exercise-specific. No nom dependency is added.
Upstream MIT copyright (c) 2014-2019 Geoffroy Couprie; full notice and pinned
license URL are recorded in THIRD_PARTY_NOTICES.md and must accompany adapted code.
-->
