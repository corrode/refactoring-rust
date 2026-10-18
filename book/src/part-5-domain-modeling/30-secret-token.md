# `secret_token`

`Credentials` holds a username and an access token for a request.
Review its diagnostic output and decide which operations callers should have.
You may change the public API in this domain-modeling exercise, but keep the username visible and preserve deliberate access to the original token.

This is an instructor-created starter inspired by the existing design of `secrecy`, not historical upstream code or an upstream vulnerability.[^source]
One starter test intentionally fails; make it pass without weakening its assertions.

Run the starter tests with `cargo test --example 30_secret_token`.

## Code to refactor

```rust,editable
{{#include ../../../examples/30_secret_token.rs:listing}}
```

<script type="text/plain" class="playground-tests" data-expected-failures="tests::debug_does_not_leak_token">
{{#include ../../../examples/30_secret_token.rs:tests}}
</script>


## Discussion

<details class="discussion-question">
<summary>What happens when a log formats these credentials?</summary>

`format!("{credentials:?}")` includes both strings, including the token.
Pretty formatting with `{credentials:#?}` exposes it too.
The derived implementation calls each field's `Debug` implementation; making the fields private would not change that output.
The `debug_does_not_leak_token` test requires the username to remain visible and the token to stay out of both formatting modes.
It fails on the original implementation.

</details>

<details class="discussion-question">
<summary>Where should the formatting policy live?</summary>

A custom `Debug` implementation for `Credentials` could omit its token.
But another struct containing the same token as a `String` would need the same care.
A dedicated token type puts the policy on the value: enclosing structs can derive `Debug` and delegate to it.
As in [rooms](24-rooms.md), the useful question is which operations belong on the new type, not whether wrapping a value is enough by itself.

</details>

<details class="discussion-question">
<summary>Which conveniences would undermine explicit access?</summary>

A public inner string would permit direct access outside the defining module.
`Deref<Target = str>` would allow implicit access through deref coercions, and a `Display` implementation that prints the string would provide another logging path.
Leaving out `Clone` and serialization avoids offering additional copying or export capabilities by default.
None of this stops a caller from deliberately reading and copying a string through an explicit accessor.

</details>

<details class="discussion-question">
<summary>Can a redaction test assert that the secret never appears in the output?</summary>

Not for every input.
An empty string occurs in every string, and a token could equal the visible username or the redaction marker.
Assert what the token field renders instead.
Test both formatting modes, including credentials nested inside another derived `Debug` implementation, and check that explicit access returns the original bytes.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### A token with explicit access

```rust,editable
{{#include ../../../solutions/30_secret_token.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/30_secret_token.rs:tests}}
</script>

`Token` owns a `String`, but its tuple field is private.
Callers outside its defining module construct it through `Token::new` and borrow its contents through `expose()`.
The custom `Debug` implementation writes a fixed marker without inspecting the string, in both normal and pretty mode.
`Credentials` can still derive `Debug`: its username remains visible, while its token delegates to the redacting implementation.
That delegation continues when another struct contains `Credentials`.

This changes the API and the diagnostic behaviour.
Replace `Credentials::new(username, token_string)` with `Credentials::new(username, Token::new(token_string))`.
Code that needs the original text now calls `credentials.token.expose()` instead of borrowing a `String` field.
The constructor moves the existing string, and `expose()` borrows it without copying.
The solution tests keep the redaction requirement and adapt construction and access to the new API.
They also cover empty strings, multiline Unicode, and tokens equal to the username or marker.

There is no `Deref`, `Display`, `Clone`, or serialization implementation on `Token`.
Those are deliberate omissions, not traits to add for convenience when adapting callers.
Rust's field privacy is a module boundary: code in the defining module and its descendants can still access the field.
Runtime formatting tests do not prove that external code cannot access private storage or use an omitted trait; those boundaries require separate compile-fail checks.

### Limits and production use

This toy addresses accidental leakage through `Debug`, not secret handling in general.
It does not zeroize memory, validate tokens, protect process memory, or stop `expose()` results from being logged, copied, or serialized.
The username remains visible by requirement; whether that is appropriate for production logs is a separate policy decision.

For production secret handling, evaluate the `secrecy` crate rather than treating this wrapper as a security implementation.[^source]
Its existing design combines redacted debugging with explicit secret access and memory-handling facilities that this exercise does not reproduce.[^implementation]
PR #1140 is a reference for restructuring `SecretBox`, not evidence that upstream previously used this starter's insecure derived `Debug`.[^pr]

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.

| Criterion | Original | `Token` |
|---|---|---|
| Type-safety | Token is an ordinary string | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Distinct type with restricted access |
| Readability | Familiar derived formatting | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Secret reads named by `expose()` |
| API design | String operations available directly | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Redaction follows the value; callers migrate |
| Performance | Owns and formats token string | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Same ownership; formats a fixed marker |
| Flexibility | Ordinary string traits and operations | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> No implicit string access or cloning |
| Error reporting | No validation errors | **N/A** No validation or error API in either version |

These implementations intentionally have different diagnostic contracts.
Readability assumes the team understands the explicit-access convention; the wrapper adds code and makes some generic APIs less convenient.
The performance comparison concerns ownership and formatting work, not a measured speed-up or a memory-protection claim.

</details>

## Takeaways For Our Own Codebase

> - Which values can reach logs through nested derived `Debug` implementations?
> - Could a type carry the formatting policy instead of repeating it at each logging site?
> - Which traits expose, copy, or export the underlying data, and which do callers actually need?
> - Search explicit-access call sites during review, including any code that copies the returned string.
> - Keep redaction tests separate from privacy checks and from claims about memory protection.

## References

- [Newtypes](https://doc.rust-lang.org/rust-by-example/generics/new_types.html): give a string its own operations and formatting policy.
- [Rooms](24-rooms.md): choose a small interface around private storage.
- [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html): derived formatting delegates to fields; custom implementations control their output.
- [Visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html): understand where a private tuple field remains accessible.
- [`secrecy`](https://docs.rs/secrecy/): production-oriented secret handling with explicit access.[^source]

[^source]: Source project: [iqlusioninc/crates, `secrecy`](https://github.com/iqlusioninc/crates/tree/main/secrecy). The starter and toy solution are instructor-created examples inspired by its existing design, not historical before-and-after upstream code.
[^implementation]: [Pinned `SecretBox` implementation, lines 141–162](https://github.com/iqlusioninc/crates/blob/f98d4ccf1e81d2fb9daaa7a5458f8064ecb563a2/secrecy/src/lib.rs#L141-L162): redacted `Debug`, constrained cloning, and explicit access through `ExposeSecret`. The pinned source's [MIT notice](https://github.com/iqlusioninc/crates/blob/f98d4ccf1e81d2fb9daaa7a5458f8064ecb563a2/secrecy/LICENSE-MIT), copyright © 2019 iqlusion, is recorded in the repository's `THIRD_PARTY_NOTICES.md`.
[^pr]: [PR #1140: rework the `SecretBox` type](https://github.com/iqlusioninc/crates/pull/1140). This restructuring is design context, not a fix for an upstream version of the insecure starter.
