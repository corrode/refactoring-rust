# 09 · `floor`

Based on [Advent of Code 2015, Day 1: Not Quite Lisp](https://adventofcode.com/2015/day/1).

> **TODO**
>
> The exercise and solutions are ready; the chapter is not yet written.

## Code to refactor

```rust,editable
{{#include ../../../examples/09_floor.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/09_floor.rs:tests}}
</script>

<details class="solution" id="a-possible-solution">
<summary><h2><span aria-hidden="true">💡</span> A possible solution</h2></summary>

```rust,editable
{{#include ../../../solutions/09_floor.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/09_floor.rs:tests}}
</script>

</details>

## Alternative: `try_fold`

<details>
<summary>Show the alternative</summary>

```rust,editable
{{#include ../../../solutions/09_floor_alt.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/09_floor_alt.rs:tests}}
</script>

</details>
