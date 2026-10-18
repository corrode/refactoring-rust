# `config_typestate`

This builder assembles a client configuration from a supplied root store.
Review its public operations and decide which calls should be available at each stage.
You may change the API in this domain-modeling exercise, but a configured build must preserve the supplied store without cloning it.
`RootStore` is an opaque teaching stand-in with a string label, not a certificate store or an implementation of trust validation.

The inspiration is [rustls][rustls], whose historical builder uses types to track configuration steps.[^rustls-builder]
The single-state starter below is our teaching example, **not historical rustls code**.
The linked source demonstrates a typed API; it is not evidence of a before-and-after refactoring PR.

## Code to refactor

```rust,editable
{{#include ../../../examples/32_config_typestate.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/32_config_typestate.rs:tests}}
</script>


## Discussion

<details class="discussion-question">
<summary>What does the type of the starter builder tell a caller?</summary>

`ConfigBuilder` can hold either `None` or `Some(roots)`.
Both values expose `build`, so the caller learns about missing roots only through a runtime error.
Calling `with_roots` returns the same type, leaving no type-level record that this step happened.

</details>

<details class="discussion-question">
<summary>What happens if a caller supplies roots twice?</summary>

The second call replaces the first store.
The starter tests record this last-setting-wins behaviour.
Removing that operation is a deliberate contract change, not a behaviour-preserving cleanup.
Decide whether callers need replacement before restricting the API.

</details>

<details class="discussion-question">
<summary>Why do the methods consume <code>self</code>?</summary>

A by-value receiver transfers ownership into the method.[^ownership]
Neither builder implements `Copy` or `Clone`.
The old binding cannot be reused after the call; the returned value carries the next available operations.
The starter already consumes its builder, but ownership alone does not prevent building too soon.

</details>

<details class="discussion-question">
<summary>Does supplying a root store prove that its contents are trustworthy?</summary>

No.
Even an empty label counts as a supplied `RootStore` here.
A type can record a completed configuration step without certifying the data passed to that step.
As with [Rooms](25-rooms.md), ownership and private storage are not content validation.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Give each stage its own operations

```rust,editable
{{#include ../../../solutions/32_config_typestate.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/32_config_typestate.rs:tests}}
</script>

`NeedsRoots` has `with_roots`, but no `build` method.
`with_roots` consumes that value and moves the supplied store into `Ready`.
Only `Ready` exposes `build`, which consumes it and returns `ClientConfig` directly.
There is no `Option` to inspect and no missing-roots error at runtime.
This is typestate: the value's type determines which operations are available.

The `roots` field in `Ready` is private, and `Ready` has no public constructor or `Default` implementation.[^privacy]
Callers outside the defining module and its descendants cannot construct it directly in safe Rust.
Code within that privacy boundary remains responsible for maintaining the construction rule.
Possessing a `Ready` is a capability to build once, not evidence that the roots are trustworthy.

### Migrate callers and tests

Replace `ConfigBuilder::default()` with `NeedsRoots::default()`.
Keep the call to `with_roots(roots)`, then remove the `unwrap`, `?`, or missing-roots match around `build()` because it now returns `ClientConfig`.
A variable annotated as `ConfigBuilder` must instead name the relevant stage, `NeedsRoots` or `Ready`.
The stages cannot be assigned interchangeably to the same variable; shadowing can give the new value the same local name.

Repeated root-setting no longer compiles because `Ready` has no `with_roots` method.
Choose the final store before entering that state, or start a fresh builder when replacement is required.
This solution intentionally changes both the early-build and repeated-setting contracts.
If replacement is part of your intended API, add a consuming `Ready::with_roots` that returns `Ready`; it need not restore the missing-roots state.

The solution tests retain content and allocation checks to verify the supplied store moves through unchanged.
They also retain an empty-store case to avoid implying content validation.
The starter's missing-roots and repeated-setting runtime tests no longer describe the solution's API.
Use compile-fail tests for building too soon and reusing a consumed value: passing runtime tests alone cannot prove those calls are rejected.

### How much typestate is enough?

The pinned rustls implementation uses `ConfigBuilder<ClientConfig, WantsVerifier>` and returns `ConfigBuilder<ClientConfig, WantsTransparencyPolicyOrClientCert>` when roots are supplied.[^rustls-builder]
Our two concrete types simplify that generic upstream transition and omit the remaining TLS configuration stages.
There is no certificate parsing, verification, or trust policy in this exercise.
Real data-dependent failures still need runtime handling even when the sequence of API calls is checked statically.

For one required value, `ClientConfig::new(roots)` could express the requirement with less code.
A staged builder earns its extra types when several operations depend on earlier choices.
Keep a runtime builder when callers genuinely need to assemble partial configuration dynamically and report missing fields together.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.

| Criterion | Original | Concrete states |
|---|---|---|
| Type-safety | Missing roots checked at runtime | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Early build unavailable |
| Readability | One type, familiar runtime check | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Named stages, more types to follow |
| API design | Same methods in both states | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Methods express prerequisites; breaking change |
| Performance | Moves store, checks `Option` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Same move; no missing-roots branch |
| Flexibility | Replacement and partial state supported | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> Fixed sequence, no replacement method |
| Error reporting | Runtime `"missing roots"` message | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Early build rejected at its call site |

The contracts differ: compile-time rejection replaces a recoverable runtime error.
Readability depends on familiarity with staged APIs, and a missing-method diagnostic still needs clear state names.
Removing a source-level branch is not a measured speed-up; the compiler may already eliminate the starter's check in a known call chain.

</details>

## Takeaways For Our Own Codebase

> - Which errors report a missing API step rather than bad external data?
> - Could a required constructor argument replace the whole builder?
> - Audit every public construction path before treating a state type as a capability.
> - Record replacement and reuse semantics before restricting available methods.
> - Keep content validation separate from proof that a configuration step occurred.

## References

- [rustls][rustls]: the real project behind the staged-builder example, not the source of our runtime starter.
- [Historical typed builder][typed-builder]: the pinned generic transition simplified here.
- [Newtypes](https://doc.rust-lang.org/rust-by-example/generics/new_types.html): distinct types and restricted operations.
- [Rooms](25-rooms.md): owning data does not validate it.
- [Method syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html): receiver choices and methods attached to specific types.

[rustls]: https://github.com/rustls/rustls
[typed-builder]: https://github.com/rustls/rustls/blob/792045ba685965edc6f872cb8fcdd3136c8f0ef8/rustls/src/client/builder.rs#L16-L31

[^rustls-builder]: [rustls historical client builder][typed-builder], pinned to commit `792045ba685965edc6f872cb8fcdd3136c8f0ef8`; this source shows the typed transition, not a verified refactoring PR. Both workshop implementations are instructor-created fixtures, not upstream excerpts. The pinned source's [MIT notice](https://github.com/rustls/rustls/blob/792045ba685965edc6f872cb8fcdd3136c8f0ef8/LICENSE-MIT), copyright © 2016 Joseph Birr-Pixton, is recorded in the repository's `THIRD_PARTY_NOTICES.md`.
[^ownership]: [Ownership and moves](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) and [method receivers](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) explain why a consuming method leaves the old binding unusable.
[^privacy]: [Visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html) describe the module boundary protecting private fields.
