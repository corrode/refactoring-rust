# 10 · `parse_port`

## The problem

The starter accepts decimal ports from 1 through 65535, but you have to reconstruct why.
It parses an `i32`, checks both bounds, casts to `u16`, and returns `Option<u16>`. The control flow
proves that the cast is safe, yet the types lose two facts that matter to callers: a
successful port is never zero, and failure has more than one cause. This refactor puts
those facts in the function's interface.

## Starter code

> **EXERCISE**
>
> Review the parsing type, range check, and return type as separate decisions. Find a type
> whose parser enforces the upper bound, then a success type that cannot contain zero.

```rust
{{#rustdoc_include ../../../examples/10_parse_port.rs:listing}}
```

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/10_parse_port.rs`. You may strengthen
> the public return type, so update the tests to check the information exposed by the new
> interface:
>
> ```sh
> cargo test --example 10_parse_port
> ```
>
> Keep the accepted range unchanged. `8080` and `65535` remain valid. Zero, malformed
> input, and out-of-range input remain invalid.

## Review

> **REVIEW**
>
> - Why parse an `i32` when the final value is a `u16`?
> - Can the destination integer's parser enforce the upper bound?
> - Is `as u16` safe here, and how close is the proof to the cast?
> - What does `Some(port)` guarantee about zero?
> - Can a caller distinguish zero, malformed text, and an out-of-range number from `None`?
> - Which failures should this API expose as separate variants?

The starter is correct for the inputs covered by its tests. Parsing an `i32` admits a
wider range, then `n < 1 || n > 65535` narrows it before `n as u16`. Because every path
to the cast passes that check, the conversion cannot truncate today. The concern is
maintenance: the proof and the conversion are separate. A later edit can loosen the
check and leave the cast untouched.

Parsing a `u16` directly removes that gap. The parser rejects negative numbers, values
above `u16::MAX`, and malformed text. There is no manual upper-bound check and no cast.
Zero still parses successfully as a `u16`, so it needs one more step. That is the job of
`NonZeroU16`.

The return type matters just as much. `Option<u16>` tells a caller whether parsing
succeeded, but not why it failed. Worse, `Some` carries no type-level promise about zero.
Any downstream function that relies on a non-zero value must trust convention or check
again. `Result<NonZeroU16, InvalidPort>` keeps the invariant in the success type and a
useful distinction in the error type.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust
{{#rustdoc_include ../../../solutions/10_parse_port.rs:listing}}
```

The first operation asks `str::parse` for a `u16`, so the target type decides which
numbers fit. Malformed text, negative input, and values above 65535 all make
`parse::<u16>()` fail. `map_err` translates any of those parser errors into
`InvalidPort::Invalid`, and `?` returns it to the caller.

That error enum is intentionally modest. `Invalid` combines syntax errors with
out-of-range numbers. The tests make the consequence concrete: both `"nope"` and
`"65536"` produce `Err(InvalidPort::Invalid)`. Callers can reject invalid input, but they
cannot respond differently to those two causes.

Once parsing succeeds, zero is the only invalid `u16`. `NonZeroU16::new(n)` returns
`None` for zero and `Some(NonZeroU16)` for every other value. `ok_or` translates that
absence into `Err(InvalidPort::Zero)`. A successful result can no longer contain zero
through Rust's safe API.

The tests construct expected values with `NonZeroU16::new(8080).unwrap()`. Those literals
are known to be non-zero, and writing the wrong literal should fail the test immediately.
The production function never unwraps untrusted input. Both failure stages become
explicit `InvalidPort` values.

</details>

## Edge cases and production concerns

> **EDGE CASE**
>
> Zero is not invalid for every use of a port. When binding a local socket, port zero can
> ask the operating system to select an available port. A configured remote destination
> usually needs a specific non-zero port.

`NonZeroU16` is the right success type only when the domain rejects zero. If this parser
later backs a bind-address API where zero carries meaning, both `NonZeroU16` and the
`Zero` error would encode the wrong policy.

The two error variants are another product decision. A form may need separate messages
for “enter digits” and “maximum value is 65535.” This enum cannot support that because
both failures become `Invalid`. Add variants or retain the parsing error when callers can
act on the difference. Keep the smaller enum when “invalid port” is enough and only zero
needs special treatment.

There is also a useful layout property, though it is not the reason for this design.
Non-zero integer types provide a niche, so `Option<NonZeroU16>` can occupy the same space
as a `u16`, using zero to represent `None`. This function returns `Result`. We choose
`NonZeroU16` for the invariant, not for that optimisation.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare the canonical `Result<NonZeroU16, InvalidPort>` interface with
> the starter's `Option<u16>` interface.

| Axis | Score | Why |
|---|---|---|
| Type-safety | wins | `NonZeroU16` excludes zero from successful results, and parsing as `u16` enforces the upper bound before construction. |
| Readability | wins | The implementation has two named stages, parse a `u16`, then reject zero, with no manual range check or cast. |
| API design | wins | The signature communicates both the non-zero invariant and the failure categories callers can observe. |
| Performance | neutral | Both versions parse once, allocate nothing, and perform constant additional work after scanning the input. |
| Flexibility | wins | Callers can handle zero separately from other invalid input and pass the success value onward without checking zero again. |
| Error reporting | wins | `Result` distinguishes zero from invalid syntax or range, while `Option` collapses every failure into `None`. |

The stronger interface makes callers handle an error enum and, at some boundaries, call
`.get()` to recover a `u16`. I'd pay that cost when non-zero is a domain rule that matters
beyond this parser. If every caller immediately extracts the integer and ignores the
error distinction, the API may be enforcing policy the application does not use.

## Quiz

### 1. Should the parser keep `i32` because it makes the range check explicit?

*Answer:* No, unless negative intermediate values mean something elsewhere in the
operation. Parsing as `u16` lets the destination type's parser enforce its bounds and
removes the cast. The tests still state the accepted range at the behaviour boundary.

### 2. Should `InvalidPort` have separate `Syntax` and `OutOfRange` variants?

*Answer:* Add them only when a caller can respond differently, for example by showing a
more specific validation message. The current design deliberately maps both causes to
`Invalid` and keeps only zero separate.

### 3. Is `Result<u16, InvalidPort>` as strong as the canonical return type?

*Answer:* No. It can report `Zero`, but its successful `u16` still permits zero at the
type level. `NonZeroU16` lets downstream functions require and preserve the invariant
without checking it again.

### 4. Should every TCP port API reject zero?

*Answer:* No. Reject zero when the value identifies a concrete non-zero port, such as a
remote destination. Accept it when the surrounding operation treats zero as a request
for an automatically assigned local port. The type must follow the domain rule.

## Follow-up questions

> **FOLLOW-UP**
>
> - Which callers need to distinguish malformed input from an out-of-range number?
> - Does zero mean “invalid,” “not configured,” or “choose a port” in your application?
> - Should downstream functions accept `NonZeroU16` so the invariant survives beyond
>   parsing?
> - What happens to exhaustive matches when you add a variant to a public error enum?
> - Which manual range checks in your codebase could become direct parsing or construction
>   of the destination type?

## Concepts recap

> **CONCEPT**
>
> - [`Option` versus `Result`](../appendix/concept-index.md#option-versus-result): choose
>   `Result` when callers need to know why an operation failed.
> - [Parsing into the destination type](../appendix/concept-index.md#parse-into-the-destination-type):
>   let the parser enforce the numeric bounds of the value you intend to return.
> - [`NonZeroU16`](../appendix/concept-index.md#nonzero-integers): represent an integer
>   whose valid set excludes zero.
> - [Type invariants](../appendix/concept-index.md#type-invariants): prevent invalid values
>   through safe construction instead of relying on repeated checks.
> - [`map_err` and `ok_or`](../appendix/concept-index.md#error-conversion-combinators):
>   translate lower-level failure representations into errors defined by your API.
