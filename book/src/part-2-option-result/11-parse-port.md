# 11 · `parse_port`

## Code to refactor

> **EXERCISE**
>
> Accept ports from 1 through 65535. Let callers distinguish zero from other invalid
> input. You may change the return type.

```rust,editable
{{#include ../../../examples/11_parse_port.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/11_parse_port.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/11_parse_port.rs:tests}}
</script>

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/11_parse_port.rs`. You may strengthen
> the public return type, so update the tests to check the information exposed by the new
> interface:
>
> ```sh
> cargo test --example 11_parse_port
> ```
>
> Keep the accepted range unchanged. `8080` and `65535` remain valid. Zero, malformed
> input, and out-of-range input remain invalid.

## Discussion

> **DISCUSSION**
>
> - Why parse an `i32` when the final value is a `u16`?
> - Can the destination integer's parser enforce the upper bound?
> - Is `as u16` safe here, and how close is the proof to the cast?
> - What does `Some(port)` guarantee about zero?
> - Can a caller distinguish zero, malformed text, and an out-of-range number from `None`?
> - Which failures should this API expose as separate variants?

The starter is correct for the inputs covered by its tests. Parsing an `i32` admits a
wider range, then `n < 1 || n > 65535` narrows it before `n as u16`. Because every path
to the cast passes that check, the conversion cannot truncate. But its safety depends
on a separate range check. A later edit could loosen the check without updating the cast.

Parsing a `u16` directly avoids the cast. The parser rejects negative numbers, values
above `u16::MAX`, and malformed text. There is no manual upper-bound check and no cast.
Zero still parses successfully as a `u16`. Constructing a `NonZeroU16` adds the check
that rejects zero.

`Option<u16>` tells a caller whether parsing succeeded, but not why it failed. It also
allows `Some(0)` at the type level. A downstream function that requires a non-zero value
must trust the parser's checks or check again. `Result<NonZeroU16, InvalidPort>` guarantees
that successful values are non-zero and distinguishes failure categories.

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Solutions</h2></summary>

```rust,editable
{{#include ../../../solutions/11_parse_port.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/11_parse_port.rs:tests}}
</script>

The first operation parses a `u16`, so the parser enforces that type's numeric bounds. Malformed text, negative input, and values above 65535 all make
`parse::<u16>()` fail. `map_err` translates any of those parser errors into
`InvalidPort::Invalid`, and `?` returns it to the caller.

`Invalid` combines syntax errors with out-of-range numbers. The tests check that both
`"nope"` and
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
later serves a bind-address API that accepts zero, both `NonZeroU16` and the `Zero`
error would enforce the wrong policy.

The error variants also depend on the application's requirements. A form may need separate messages
for “enter digits” and “maximum value is 65535.” This enum cannot support that because
both failures become `Invalid`. Add variants or retain the parsing error when callers can
act on the difference. Keep the smaller enum when “invalid port” is enough and only zero
needs special treatment.

Non-zero integer types also provide an unused value, called a niche.
`Option<NonZeroU16>` can occupy the same space as a `u16`, using zero to represent `None`.
This function returns `Result`, not `Option`. It uses `NonZeroU16` to guarantee a non-zero
value, not for that optimisation.

## Trade-offs

Compared with the original: **+** advantage, **=** comparable, **−** disadvantage,
**N/A** not applicable. Readability depends on familiarity.

| Criterion | Original: `Option<u16>` | `Result<NonZeroU16, InvalidPort>` (main) |
|---|---|---|
| Type-safety | Zero excluded by runtime convention | + Success type excludes zero |
| Readability | Range check before narrowing cast | + Direct parsing, no cast |
| API design | Success or undifferentiated absence | + Non-zero success, named failures |
| Performance | One parse, no allocation | = One parse, no allocation |
| Flexibility | Callers receive plain integers | + Non-zero values compose downstream |
| Error reporting | Every failure becomes `None` | + Distinguishes zero from other failures |

Callers must handle `InvalidPort` and use `.get()` where a `u16` is needed.
Syntax and range errors still share `Invalid`. Rejecting zero is an existing domain
rule, not a correctness fix; this type would be wrong for bind APIs that accept zero.

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

## Important Concepts

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
