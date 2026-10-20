# `floor`

This function follows a sequence of instructions to find the final floor, starting at zero.
`(` moves up one floor and `)` moves down one.
Can you express the calculation more directly?
Keep the signature, result, and error for the first invalid instruction unchanged.

This exercise comes from [Advent of Code 2015, Day 1: Not Quite Lisp](https://adventofcode.com/2015/day/1).


> **TODO**
>
> The exercise and solutions are ready.
> The discussion and remaining sections still need writing.

## Code to refactor

```rust,editable
{{#include ../../../examples/10_floor.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/10_floor.rs:tests}}
</script>

<div class="playground-local-setup">

**Run locally**

From the repository root:

```sh
cargo test --example 10_floor
```

</div>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Using `sum`

```rust,editable
{{#include ../../../solutions/10_floor.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/10_floor.rs:tests}}
</script>

### Using `try_fold`

```rust,editable
{{#include ../../../solutions/10_floor_alt.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/10_floor_alt.rs:tests}}
</script>

</details>
