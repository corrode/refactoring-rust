# `buffer_policy`

`BufferConfig` selects how a read buffer is sized.
Review its representation while keeping the setter signatures and behaviour unchanged.
You may change the public type in this domain-modeling exercise; adapt code that constructs or inspects it directly.
This exercise is inspired by hyper's buffer configuration.[^source]

## Code to refactor

```rust,editable
{{#include ../../../examples/29_buffer_policy.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/29_buffer_policy.rs:tests}}
</script>


## Discussion

<details class="discussion-question">
<summary>Are these two independent options?</summary>

No.
Each setter clears the other option.
Starting from the default, the setters produce three configurations: adaptive sizing without an explicit maximum, an exact size, or adaptive sizing with an explicit maximum.
The two `Option` fields also permit a fourth combination, with both set, which these setters never produce.

</details>

<details class="discussion-question">
<summary>Is this a bug in hyper?</summary>

The historical setters already enforce mutual exclusion by clearing the other field.
This exercise does not demonstrate an upstream bug.
It extracts that policy into a small type with public fields for inspection; hyper's original fields were private inside its connection builder.
The question is whether the representation can express the rule instead of requiring each mutation path to maintain it.

</details>

<details class="discussion-question">
<summary>Does setter order matter?</summary>

Yes.
Setting an exact size and then a maximum selects adaptive sizing with that maximum.
Reversing those calls selects the exact size.
`exact_size(None)` clears both settings, including any previous maximum.
It is a reset, not a request to leave the current configuration alone.

</details>

<details class="discussion-question">
<summary>Which numeric checks belong to this refactor?</summary>

Keep the existing ones.
`max_size` accepts 8192 and above, and panics below that boundary before changing either field.
`exact_size` accepts every `usize`, including zero.
Accepting a configuration value here is not a promise that allocating or using a buffer of that size will succeed.
Adding a minimum exact size would change the contract rather than preserve it.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Name the three policies

```rust,editable
{{#include ../../../solutions/29_buffer_policy.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/29_buffer_policy.rs:tests}}
</script>

An enum holds one policy at a time.
`Adaptive` means no explicit size or maximum, not an unlimited buffer.
`Exact(size)` and `AdaptiveWithMax(max)` each carry the value their policy needs.
There is no variant for selecting both.
This enum is a workshop redesign, not a claimed upstream refactor.

Each setter replaces the whole value, so it no longer needs to clear a second field.
The assertion still runs before assignment.
If it panics, the previous policy remains intact.

### Callers and tests

Code using `BufferConfig::default()` and the two setters stays unchanged.
Code reading fields must instead match on the enum; struct literals become the corresponding variants.
A literal with both options set has no direct replacement: the caller must choose a policy.
That is a deliberate API change, even though setter behaviour stays the same.

The solution tests replace field inspection with an exhaustive match in the `policy` helper.
The assertions stay the same: default behaviour, ordinary sizes, repeated updates, both setter orders, reset from every policy, zero exact size, and the minimum maximum.
They also check `usize::MAX` without allocating a buffer.
For invalid maxima, the tests catch an unwinding panic and verify that all three starting policies remain unchanged.
This is a test technique, not a recommendation to recover from panics in application code.

### What the enum does not guarantee

Mutual exclusion and numeric validation are different rules.
A public `AdaptiveWithMax(1)` is still constructible without calling the setter.
The enum prevents conflicting policies, but it does not enforce the minimum maximum on every construction path.
If that guarantee matters, hide construction behind a wrapper or use a validated payload type.
That would be a further API design decision.

The [rooms exercise](25-rooms.md) makes a related distinction: naming a type does not automatically validate its contents.
Here the enum enforces one specific invariant while leaving numeric checks in the setter.
Neither implementation allocates a buffer or implements adaptive growth.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | Policy enum |
|---|---|---|
| Type-safety | Setters maintain mutual exclusion | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Conflicting policies unrepresentable |
| Readability | Meaning spread across two fields | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Each policy has a name |
| API design | Public fields allow conflicting settings | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Explicit choices; field users must migrate |
| Performance | Constant-time updates, no allocation | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Constant-time replacement, no allocation |
| Flexibility | Two independently accessible fields | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> New variants require exhaustive matches to change |
| Error reporting | Assertion for an invalid maximum | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Same assertion and message |

No speed or memory-layout improvement is assumed here.
The benefit is expressing mutual exclusion in the type.
Keep the original representation if its private mutation paths are easy to audit and changing it would cost more than it helps.

</details>

## Takeaways For Our Own Codebase

> - Which setters repeatedly clear another field?
>   Check whether the fields represent separate options or alternative policies.
> - Record reset behaviour and both setter orders before changing a representation.
> - Validate before mutation when rejected input must leave the old state intact.
> - Separate what a type guarantees from what its setters check.
>   Inspect direct construction paths too.

## References

- [Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html): represent mutually exclusive choices with their associated data.
- [Pattern matching](https://doc.rust-lang.org/book/ch06-02-match.html): inspect a policy and handle every variant.
- [Rooms](25-rooms.md): distinguish an API redesign from validation of its contents.
- [hyper](https://github.com/hyperium/hyper): inspiration for this exercise.[^source]

[^source]: Inspired by [hyper's buffer configuration setters](https://github.com/hyperium/hyper/blob/73bff4e98c372ce04b006370c0b0d2af29ea8718/src/client/conn.rs#L616-L642).
    The enum solution is a workshop redesign.
