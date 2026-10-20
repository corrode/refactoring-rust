# `typed_ids`

`Engine` stores variables and declarations in separate vectors and looks them up by ID.
Review its public interface while keeping successful lookups and the `None` result for missing entries.
You may change the API in this domain-modeling exercise, including how callers construct IDs.
This workshop-created fixture draws on an ID refactor in [Nushell](https://github.com/nushell/nushell), not on a copy of its engine.[^provenance]

One test intentionally fails; make it pass without weakening its assertion.

## Code to refactor

```rust,editable
{{#include ../../../examples/30_typed_ids.rs:listing}}
```

<script type="text/plain" class="playground-tests" data-expected-failures="tests::variable_and_declaration_ids_are_distinct">
{{#include ../../../examples/30_typed_ids.rs:tests}}
</script>

<div class="playground-local-setup">

**Run locally**

From the repository root:

```sh
cargo test --example 30_typed_ids
```

</div>


## Discussion

<details class="discussion-question">
<summary>What does a type alias enforce here?</summary>

`VarId` and `DeclId` both mean `usize` to the compiler.
The names document intent but don't create distinct types.
A caller can pass a declaration ID to `variable`, or a variable ID to `declaration`, without a compiler error.
The starter test `variable_and_declaration_ids_are_distinct` asserts that the categories have different types.
It fails because both aliases refer to `usize`.

</details>

<details class="discussion-question">
<summary>Does returning <code>Option</code> catch an ID mix-up?</summary>

Only when the numeric index happens to be out of range.
If both vectors contain an entry at that index, the lookup returns a value from the wrong category.
Bounds checking cannot tell you which collection the caller meant.

</details>

<details class="discussion-question">
<summary>What should change for callers?</summary>

The API should require callers to distinguish the two categories before making a lookup.
Changing an alias into a distinct type breaks existing calls that pass raw integers.
Update the places that create IDs and carry the distinction through intermediate functions, rather than repeatedly converting back to `usize`.
The lookup results can stay borrowed `Option<&str>` values.

</details>

<details class="discussion-question">
<summary>Would distinct ID types prove that an entry exists?</summary>

No.
A category and a valid index are different guarantees.
An ID can still be out of range, and an ID used with one engine might refer to unrelated data in another.
Decide which guarantee you need before designing constructors that appear to promise more.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Give each category its own type

```rust,editable
{{#include ../../../solutions/30_typed_ids.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/30_typed_ids.rs:tests}}
</script>

The two tuple structs are different types even though each contains a `usize`.
`variable` accepts only `VarId`; `declaration` accepts only `DeclId`.
Each method extracts the numeric index with `.0` and performs the same checked lookup as before.
No extra allocation or traversal is involved.

`Engine::new` stays public and takes ownership of both vectors, so callers outside this module can construct an engine without access to its private fields.
The ID fields are public too: callers write `VarId(1)` or `DeclId(0)`.
There is no validation to hide behind a constructor, so separate `new` and `get` methods would add little to this example.
`Copy` and `Clone` let callers reuse an ID; `Debug`, equality, and hashing support diagnostics, comparisons, and map keys.

### Migrate callers and tests

Replace `let variable: VarId = 1` with `let variable = VarId(1)`, and construct declaration IDs with `DeclId`.
Keep those types in parameters and return values along the lookup path.
The solution's tests use these constructors while preserving the successful, empty, and out-of-range lookup expectations.

With the solution's `Engine`, `VarId`, and `DeclId` in scope, these swapped calls are deliberately invalid:

```rust,ignore,noplayground
let engine = Engine::new(vec!["count".into()], vec!["print".into()]);
let variable = VarId(0);
let declaration = DeclId(0);
engine.variable(declaration); // Expected VarId, found DeclId.
engine.declaration(variable); // Expected DeclId, found VarId.
```

The starter accepts both calls; the solution rejects them at compile time.
This is a contextual illustration, not a standalone test with missing imports.
The unit test comparing `TypeId` values checks that the types differ, but it does not replace a compile-fail regression test for these calls.
You can still explicitly write `VarId(declaration.0)`.
The newtype stops accidental interchange, not a caller deliberately relabeling an integer.

### What these IDs do not guarantee

`VarId(usize::MAX)` is constructible and returns `None` when used for a lookup.
A `VarId(0)` can also be passed to two engines and retrieve different variables.
The solution tests both limitations.
The types identify categories, not engine instances or live entries.

If a real engine removes entries and reuses slots, an old ID can refer to a new entry in the same category.
Generation counters can help detect stale IDs; instance identity needs its own design.
Neither follows from wrapping a `usize`, and neither belongs in this exercise's lookup contract.

### When a generic ID is useful

Nushell's actual refactor replaced integer aliases with a generic `Id<T>` containing an index and `PhantomData<T>`.
Its `VarId` and `DeclId` aliases select different marker types, so the resulting generic instantiations are distinct types.[^provenance]
An alias can therefore name a distinct type without creating that distinction itself.

This solution uses separate tuple structs as a teaching simplification.
For two categories, I would keep the distinction visible without introducing a generic abstraction.
With many categories and shared ID operations, a generic wrapper can reduce repetition.
The marker still identifies a category, not a particular engine or a valid slot.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | Separate newtypes |
|---|---|---|
| Type-safety | Categories interchangeable | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Rejects swapped ID categories |
| Readability | Names document intent | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Construction makes category explicit |
| API design | Accepts any `usize` | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Enforces intent; caller migration required |
| Performance | Checked vector lookup | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Same lookup, no extra allocation |
| Flexibility | Integers work in either lookup | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> Shared integer helpers need adaptation |
| Error reporting | Missing entries return `None` | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Category mismatch reported at compile time |

The error-reporting advantage concerns category mistakes only.
A missing entry still returns `None`, and a same-category ID from another engine can still return an unrelated value.
This API deliberately removes interchangeability; it is not a drop-in replacement for the aliases.

</details>

## Takeaways For Our Own Codebase

> - Which integer aliases represent categories that callers must not mix?
> - Carry domain IDs through intermediate APIs instead of stripping their types at each boundary.
> - Separate category safety from bounds checks, stale-ID detection, and instance identity.
> - Test runtime lookup behaviour and compile-time rejection as different contracts.
> - Introduce a generic ID wrapper when shared operations justify it, not just because two types contain an integer.

## References

- [Newtypes](https://doc.rust-lang.org/rust-by-example/generics/new_types.html): distinguish values that share a representation.
- [Rooms](25-rooms.md): another domain API change, and why wrapping data does not validate it.
- [Type aliases](https://doc.rust-lang.org/reference/items/type-aliases.html): aliases name existing types rather than creating new ones.
- [`PhantomData`](https://doc.rust-lang.org/std/marker/struct.PhantomData.html): represent a type parameter without storing a value of that type.
- [Nushell PR #13832](https://github.com/nushell/nushell/pull/13832): the production refactor that inspired this exercise.

[^provenance]: [Nushell](https://github.com/nushell/nushell) replaced integer ID aliases with a generic marker-based ID in [PR #13832](https://github.com/nushell/nushell/pull/13832). The pinned source shows [`Id<T>` and `PhantomData`](https://github.com/nushell/nushell/blob/f0c83a445977c894c4c46ca9e1c97e3372eddd69/crates/nu-protocol/src/id.rs#L7-L33) and the [category aliases](https://github.com/nushell/nushell/blob/f0c83a445977c894c4c46ca9e1c97e3372eddd69/crates/nu-protocol/src/id.rs#L81-L87). The `Engine` fixture and separate tuple-newtype solution here were created for the workshop; they are not excerpts from Nushell. The pinned source is [MIT-licensed](https://github.com/nushell/nushell/blob/f0c83a445977c894c4c46ca9e1c97e3372eddd69/LICENSE), copyright © 2019–2023 The Nushell Project Developers; its notice is recorded in the repository's `THIRD_PARTY_NOTICES.md`.
