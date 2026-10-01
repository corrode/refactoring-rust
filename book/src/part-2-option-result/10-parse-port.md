# 10 · `parse_port`

## The problem

A port parser needs to accept decimal values from 1 through 65535. The starter parses an
`i32`, checks both bounds by hand, casts the value to `u16`, and returns `Option<u16>`.
The tests pass, but the types forget two useful facts: a successful port is never zero,
and a failure can have more than one cause. This refactor moves both facts into the
function's interface.

## Starter code

> **EXERCISE**
>
> Review the parsing type, the range check, and the return type separately. Look for a
> type that enforces the upper bound during parsing and a success type that cannot contain
> zero.

```rust
{{#include ../../../examples/10_parse_port.rs}}
```

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/10_parse_port.rs`. You may strengthen
> the public return type, so update the tests to check the information the new interface
> provides:
>
> ```sh
> cargo test --example 10_parse_port
> ```
>
> Preserve the accepted range. Keep `8080` and `65535` valid, reject zero, and reject
> malformed or out-of-range input.

## Review

> **REVIEW**
>
> - Why does the starter parse into `i32` when the final value is `u16`?
> - Can the parser for the destination integer type enforce the upper bound for us?
> - Is the `as u16` cast safe here, and how far away is the proof that makes it safe?
> - What guarantee does `Some(port)` give a caller about zero?
> - Can a caller tell whether `None` means zero, malformed text, or an out-of-range
>   number?
> - Which failures deserve distinct error variants in this API?

The starter is correct for its tested inputs. Parsing as `i32` accepts a wider range, then
`n < 1 || n > 65535` narrows that range before the cast. Because the check dominates
`n as u16`, the cast cannot truncate in the current control flow. The maintenance risk is
that the proof and the conversion are separate operations. A later edit can weaken the
check while leaving the cast unchanged.

Parsing directly as `u16` lets the parser enforce the numeric bounds. Negative values and
values above `u16::MAX` fail alongside malformed input, so the manual upper-bound check
and cast disappear. Zero still parses as a `u16`, which is where `NonZeroU16` becomes
useful.

The return type deserves equal attention. `Option<u16>` tells the caller only whether
parsing succeeded. Even after `Some` is returned, the type itself still permits zero, so
a caller cannot carry the parser's guarantee into another function without relying on
convention. `Result<NonZeroU16, InvalidPort>` records the invariant in the success type
and keeps a useful distinction in the error type.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust
{{#include ../../../solutions/10_parse_port.rs}}
```

The first line of `parse_port` asks `str::parse` for a `u16`. The target type determines
which values are accepted. If the text is malformed, negative, or above 65535,
`parse::<u16>()` returns an error. `map_err` converts every such parsing failure into
`InvalidPort::Invalid`, and `?` returns it to the caller.

That error design is intentionally small. The current `Invalid` variant combines syntax
errors and out-of-range numbers. The tests make this visible: both `"nope"` and `"65536"`
produce `Err(InvalidPort::Invalid)`. A caller can react to invalid input, but it cannot
tell those two causes apart.

After parsing, zero is the only rejected `u16`. `NonZeroU16::new(n)` returns
`Option<NonZeroU16>`: `None` for zero and `Some` for every other `u16`. `ok_or` changes
that absence into `Err(InvalidPort::Zero)`. A successful result now contains a value that
cannot be zero through Rust's safe API.

The tests use `NonZeroU16::new(8080).unwrap()` to construct expected values. Those
literals are known to be non-zero, and a mistake should fail the test immediately. The
production function does not unwrap untrusted input. It converts both failure stages
into explicit `InvalidPort` values.

</details>

## Edge cases and production concerns

> **EDGE CASE**
>
> Whether zero is invalid depends on what the port represents. Binding a local socket to
> port zero can ask the operating system to choose an available port. A configured remote
> destination usually needs a specific non-zero port.

`NonZeroU16` is correct only when the domain rejects zero. If this parser later serves a
bind-address API where zero has meaning, the success type and `Zero` error would encode
the wrong policy.

The two-variant error also reflects a product decision. A form might need to tell users
“enter digits” and “maximum value is 65535” separately. This enum cannot do that because
both cases become `Invalid`. Add more variants or retain the parsing error when callers
can act on the distinction. Keep the smaller enum when “invalid port” is enough and zero
alone needs special treatment.

As a layout side note, non-zero integer types provide a niche that Rust can use for
`Option<NonZeroU16>`. That option can occupy the same space as a `u16`, with zero
representing `None`. This function returns `Result`, so the niche is not the reason to
choose `NonZeroU16` here. The enforced invariant is.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare the canonical `Result<NonZeroU16, InvalidPort>` interface with
> the starter's `Option<u16>` interface.

| Axis | Score | Why |
|---|---|---|
| Type-safety | wins | `NonZeroU16` makes zero unrepresentable in successful results, and parsing as `u16` enforces the upper bound before construction. |
| Readability | wins | The implementation follows two named stages, parse a `u16`, then reject zero, without a manual range check or cast. |
| API design | wins | The signature communicates both the non-zero invariant and the failure categories that callers can observe. |
| Performance | neutral | Both versions parse once, allocate nothing, and perform constant additional work after scanning the input. |
| Flexibility | wins | Callers can match zero separately from other invalid input and pass the success value onward without checking zero again. |
| Error reporting | wins | `Result` distinguishes zero from invalid syntax or range, while `Option` collapses every failure into `None`. |

The stronger interface asks callers to handle an error enum and work with `NonZeroU16`.
That is extra code at boundaries that only want a `u16`. I would accept that cost when
non-zero is a domain rule used beyond this function. If every caller immediately calls
`.get()` and discards the error distinction, the interface may be carrying policy that
the application does not use.

## Quiz

### 1. Should the parser keep `i32` because it makes the range check explicit?

*Answer:* No, unless negative intermediate values have meaning elsewhere in the
operation. Parsing as `u16` delegates the destination type's bounds to the parser and
removes the cast. The tests still state the accepted range at the behaviour boundary.

### 2. Should `InvalidPort` have separate `Syntax` and `OutOfRange` variants?

*Answer:* Add them only when a caller can respond differently, such as showing a more
specific validation message. The current design deliberately maps both causes to
`Invalid` and keeps only zero distinct.

### 3. Is `Result<u16, InvalidPort>` as strong as the canonical return type?

*Answer:* No. It can report `Zero`, but a successful `u16` still permits zero at the type
level. `NonZeroU16` lets downstream functions require and preserve the invariant without
rechecking it.

### 4. Should every TCP port API reject zero?

*Answer:* No. Reject zero when the value names a concrete non-zero port, such as a remote
destination. Permit it when the surrounding operation uses zero as a request for an
automatically assigned local port. The type must follow that domain rule.

## Follow-up questions

> **FOLLOW-UP**
>
> - Which callers need to distinguish malformed input from an out-of-range number?
> - Does zero mean “invalid,” “not configured,” or “choose a port” in your application?
> - Should downstream functions accept `NonZeroU16` so the invariant survives beyond
>   parsing?
> - How would adding a variant to a public error enum affect callers with exhaustive
>   matches?
> - Which other manual range checks in your codebase could be replaced by parsing or
>   constructing the destination type directly?

## Concepts recap

> **CONCEPT**
>
> - [`Option` versus `Result`](../appendix/concept-index.md#option-versus-result): choose
>   `Result` when callers need information about why an operation failed.
> - [Parsing into the destination type](../appendix/concept-index.md#parse-into-the-destination-type):
>   let the parser enforce the numeric bounds of the value you intend to return.
> - [`NonZeroU16`](../appendix/concept-index.md#nonzero-integers): represent an integer
>   whose valid set excludes zero.
> - [Type invariants](../appendix/concept-index.md#type-invariants): make an invalid state
>   unavailable through safe construction rather than relying on repeated checks.
> - [`map_err` and `ok_or`](../appendix/concept-index.md#error-conversion-combinators):
>   translate lower-level failure representations into errors defined by your API.
