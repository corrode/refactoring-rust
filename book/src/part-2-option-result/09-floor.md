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

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Solutions</h2></summary>

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
