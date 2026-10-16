# `floor`

Follow the instructions to find the final floor, starting at zero.
`(` moves up one floor and `)` moves down one.
Refactor the loop while preserving the signature, result, and error for the first invalid instruction.

This exercise comes from [Advent of Code 2015, Day 1: Not Quite Lisp](https://adventofcode.com/2015/day/1).
Run its tests with `cargo test --example 09_floor`.

> **TODO**
>
> The exercise and solutions are ready; the discussion and remaining sections still need writing.

## Code to refactor

```rust,editable
{{#include ../../../examples/09_floor.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/09_floor.rs:tests}}
</script>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Using `sum`

```rust,editable
{{#include ../../../solutions/09_floor.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/09_floor.rs:tests}}
</script>

### Using `try_fold`

```rust,editable
{{#include ../../../solutions/09_floor_alt.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/09_floor_alt.rs:tests}}
</script>

</details>
