---
layout: default
---

# 09 · `floor`

Start on floor zero. `(` takes you up one floor; `)` takes you down.

```rust
pub fn floor(instructions: &str) -> Result<i64, String> {
    let mut floor = 0;
    for instruction in instructions.chars() {
        match instruction {
            '(' => floor += 1,
            ')' => floor -= 1,
            c => return Err(format!("Invalid instruction: {c}")),
        }
    }
    Ok(floor)
}
```

<div class="mt-4 text-sm opacity-60">

Based on [Advent of Code 2015, Day 1: Not Quite Lisp](https://adventofcode.com/2015/day/1).

</div>

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 09_floor</code>
</div>

<!--
Keep the Result<i64, String> signature and stop at the first invalid character.
Try (()) for Ok(0), (()(()( for Ok(3), and ))) for Ok(-3).
Let participants try it before showing the solution.
-->

---

# 09 · Review

- Can an iterator sum steps without storing them in a `Vec`?
- What should happen at the first invalid character?
- What should empty input return?
- Which version would you rather maintain?

<!--
In parse_values, collect stopped at the first Err. Can sum do that too?
I'd be happy to keep the loop. Ask participants whether knowing this implementation
of sum changes their preference.
-->

---

# 09 · Sum the results directly

```rust
pub fn floor(instructions: &str) -> Result<i64, String> {
    instructions
        .chars()
        .map(|c| match c {
            '(' => Ok(1),
            ')' => Ok(-1),
            _ => Err(format!("Invalid instruction: {c}")),
        })
        .sum()
}
```

- `sum` adds successful steps and stops at the first error.
- No intermediate collection. Empty input returns `Ok(0)`.

<!--
The return type selects Sum<Result<i64, String>> for Result<i64, String>.
sum adds the numbers inside Ok and returns the first Err. Like collect in
parse_values, its behavior depends on the return type. There's no Vec here.
-->

---

# 09 · Alternative: `try_fold`

```rust
pub fn floor(instructions: &str) -> Result<i64, String> {
    instructions
        .chars()
        .try_fold(0, |floor, instruction| match instruction {
            '(' => Ok(floor + 1),
            ')' => Ok(floor - 1),
            c => Err(format!("Invalid instruction: {c}")),
        })
}
```

- The closure receives the current floor and returns the next one.
- An `Err` stops the fold.

<!--
Here, each step is +1 or -1 regardless of the current floor, so sum is enough.
try_fold lets the next step depend on the floor we're on.
If someone suggests scan, ask what they'd do with the intermediate floors.
Part two of the puzzle needs them: when do we first enter the basement?
-->
