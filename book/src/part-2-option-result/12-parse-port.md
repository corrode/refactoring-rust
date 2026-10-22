# `parse_port`

This parser accepts ports from 1 through 65535.
Refactor it without changing the function signature or accepted inputs.
Keep the accepted range unchanged: zero, malformed text, and out-of-range numbers remain invalid.
Afterward, consider an alternative API that distinguishes zero from other invalid input.

## Code to refactor

```rust,editable
{{#include ../../../examples/12_parse_port.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/12_parse_port.rs:tests}}
</script>

<div class="playground-local-setup">

**Run locally**

From the repository root:

```sh
cargo test --example 12_parse_port
```

</div>


## Discussion

<details class="discussion-question">
<summary>Why parse an <code>i32</code> for a <code>u16</code> result?</summary>

There is no need for the wider type here.
The starter parses an `i32`, then checks `n < 1 || n > 65535` before converting with `n as u16`.
That works for the inputs covered by its tests, but parsing a `u16` directly avoids the cast.

</details>

<details class="discussion-question">
<summary>Can the parser check the upper bound?</summary>

Yes, the `u16` parser rejects values above `u16::MAX`, along with negative numbers and malformed text.
You don't need a manual upper-bound check or a cast.
Zero still parses successfully as a `u16`.
You still need to reject zero.
The main solution uses `Option::filter`; the alternative constructs a `NonZeroU16`.

</details>

<details class="discussion-question">
<summary>Is <code>as u16</code> safe here?</summary>

Yes, every path to the cast passes the range check, so the conversion cannot truncate.
But the cast relies on that separate check.
A later edit could loosen the check without updating the cast.

</details>

<details class="discussion-question">
<summary>Does <code>Some(port)</code> rule out zero?</summary>

No, `Option<u16>` allows `Some(0)`.
If a downstream function requires a non-zero value, it must trust the parser's checks or check again.
`Result<NonZeroU16, InvalidPort>` guarantees that successful values are non-zero and distinguishes failure categories.

</details>

<details class="discussion-question">
<summary>Does <code>None</code> tell us why parsing failed?</summary>

No, `Option<u16>` only tells you whether parsing succeeded.
Zero, malformed text, and out-of-range numbers all become `None`.

</details>

<details class="discussion-question">
<summary>Which failures need separate variants?</summary>

If callers need to distinguish zero from other invalid input, give zero its own variant.
That requires an API change; the main solution keeps `Option<u16>`.
Use `InvalidPort::Zero` for zero and `InvalidPort::Invalid` for the remaining failures.
Malformed text and out-of-range numbers still share `Invalid`.
Add variants or retain the parsing error when callers need to act on that difference.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

```rust,editable
{{#include ../../../solutions/12_parse_port.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/12_parse_port.rs:tests}}
</script>

Parse directly into `u16` to let the parser check the numeric bounds.
Malformed text, negative input, and values above 65535 all make `parse::<u16>()` fail.
`ok()` turns a parse error into `None`, preserving the original error policy.
`filter` keeps a parsed value only when it isn't zero.
The function still returns `Option<u16>`, so callers need no changes.
The tests check the original signature, the range boundaries, and accepted syntax such as a leading plus sign.

### Alternative: return a non-zero value and an error

If callers need more guarantees or error information, change the return type:

```rust,editable
{{#include ../../../solutions/12_parse_port_alt.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/12_parse_port_alt.rs:tests}}
</script>

`map_err` translates any parser error into `InvalidPort::Invalid`, and `?` returns it to the caller.

`Invalid` combines syntax errors with out-of-range numbers.
The tests check that both `"nope"` and `"65536"` produce `Err(InvalidPort::Invalid)`.
Callers can reject invalid input, but they cannot respond differently to those two causes.

Once parsing succeeds, zero is the only invalid `u16`.
`NonZeroU16::new(n)` returns `None` for zero and `Some(NonZeroU16)` for every other value.
`ok_or` turns `None` into `Err(InvalidPort::Zero)`.
Rust's safe API prevents a successful result from containing zero.

The tests construct expected values with `NonZeroU16::new(8080).unwrap()`.
Those literals are known to be non-zero, and writing the wrong literal should fail the test immediately.
The production function never unwraps untrusted input.
Both failure stages become explicit `InvalidPort` values.

`parse::<u16>()` already uses a standard trait: `FromStr`.
If ports later need their own domain type, implement `FromStr` for that type rather than adding only an inherent parsing method.
That lets callers use `text.parse::<Port>()` and lets generic parsers use the same interface.
The implementation can expose the domain's error type through `FromStr::Err`.
Keep the free function as a wrapper if existing callers need it; introducing a `Port` type is not required for this exercise.

### Edge cases and production concerns

> **EDGE CASE**
>
> Zero is not invalid for every use of a port.
> When binding a local socket, port zero can ask the operating system to select an available port.
> A configured remote destination usually needs a specific non-zero port.

`NonZeroU16` is the right success type only when the domain rejects zero.
If this parser later serves a bind-address API that accepts zero, both `NonZeroU16` and the `Zero` error would enforce the wrong policy.

Does a form need separate messages for “enter digits” and “maximum value is 65535”?
This enum cannot distinguish them because both failures become `Invalid`.
Add variants or retain the parsing error when callers can act on the difference.
Keep the smaller enum when “invalid port” is enough and only zero needs special treatment.

Non-zero integer types also provide an unused value, called a niche.
`Option<NonZeroU16>` can occupy the same space as a `u16`, using zero to represent `None`.
The alternative returns `Result`, not `Option`.
It uses `NonZeroU16` to guarantee a non-zero value, not for that optimisation.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | `Option<u16>` (main) | `Result<NonZeroU16, InvalidPort>` (alternative) |
|---|---|---|---|
| Type-safety | Zero excluded by runtime convention | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Zero excluded by runtime check | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Success type excludes zero |
| Readability | Range check before narrowing cast | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Direct parsing, no cast | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Direct parsing, no cast |
| API design | Success or undifferentiated absence | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Preserves success-or-absence API | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Non-zero success, named failures |
| Performance | One parse, no allocation | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> One parse, no allocation | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> One parse, no allocation |
| Flexibility | Callers receive plain integers | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Callers receive plain integers | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Non-zero values compose downstream |
| Error reporting | Every failure becomes `None` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Every failure still becomes `None` | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Distinguishes zero from other failures |

Only callers of the alternative must handle `InvalidPort` and use `.get()` where a `u16` is needed.
Syntax and range errors still share `Invalid`.
The original already rejects zero, so the new type enforces an existing domain rule rather than fixing a bug.
Don't use it for bind APIs that accept zero.

</details>

## Takeaways For Our Own Codebase

> - Which callers need to distinguish malformed input from an out-of-range number?
>   Keep enough error information for them to act on that difference.
> - Does zero mean “invalid,” “not configured,” or “choose a port” in your application?
> - Could downstream functions accept `NonZeroU16` instead of checking for zero again?
> - Before adding a public error variant, check which callers match the enum exhaustively.
> - Look for manual range checks you could replace by parsing or constructing the destination type directly.

## References

- [`Option` versus `Result`](../appendix/concept-index.md#option-versus-result): choose `Result` when callers need to know why an operation failed.
- [Parsing into the destination type](../appendix/concept-index.md#parse-into-the-destination-type): let the parser enforce the numeric bounds of the value you intend to return.
- [`NonZeroU16`](../appendix/concept-index.md#nonzero-integers): represent an integer whose valid set excludes zero.
- [Type invariants](../appendix/concept-index.md#type-invariants): prevent invalid values through safe construction instead of relying on repeated checks.
- [`map_err` and `ok_or`](../appendix/concept-index.md#error-conversion-combinators): translate lower-level failure representations into errors defined by your API.
- [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html): provide a string parser that callers can use through `str::parse`.
- [Google Rust Style Guide: standard library traits](https://google.github.io/styleguide/rustguide.html#implement-known-standard-library-traits-instead-of-custom-methods): use an established trait when its contract matches the operation, rather than offering only a custom method.
