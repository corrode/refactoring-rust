---
layout: default
zoom: 0.85
---

# 20 · `apply_commands`

<div class="opacity-80 mb-4">
Apply a sequence of <code>Command</code>s to strings.
</div>

```rust
pub enum Command { Uppercase, Trim, Append(usize) }

pub fn apply_commands(input: Vec<(String, Command)>) -> Vec<String> {
    let mut result = Vec::new();
    for i in 0..input.len() {
        let item = input[i].clone();
        let s = item.0;
        let c = item.1;

        if let Command::Uppercase = c {
            result.push(s.to_uppercase());
        } else if let Command::Trim = c {
            result.push(s.trim().to_string());
        } else if let Command::Append(n) = c {
            let mut temp_string = s;
            for _ in 0..n {
                temp_string.push_str("bar");
            }
            result.push(temp_string);
        }
    }
    result
}
```

<div class="absolute top-25 right-12 text-sm opacity-60">
<code>cargo test --example 20_transformer</code>
</div>

<!--
Discoveries:
  - The index loop clones elements from a `Vec` we already own.
  - Chained `if let` on an enum should be `match`, with exhaustiveness checking.
  - Manual loop appending "bar" n times is `"bar".repeat(n)` or `s.push_str(&"bar".repeat(n))`.
  - "Map every element through a function" is `iter.map(...).collect()`.
-->

---

# 20 · Review

- We own <code>input</code>. Why are we cloning each element?
- A chain of <code>if let</code> on the same enum… what's the better tool?
- We push <code>"bar"</code> in a loop, <code>n</code> times. Does <code>str</code> have a helper for that?
- Build a <code>Vec</code> by walking another <code>Vec</code>: that's almost always <code>iter().map(…).collect()</code>.


---

# 20 · Possible solution

```rust
pub fn apply_commands(input: Vec<(String, Command)>) -> Vec<String> {
    input
        .into_iter()
        .map(|(s, cmd)| match cmd {
            Command::Uppercase => s.to_uppercase(),
            Command::Trim => s.trim().to_string(),
            Command::Append(n) => s + &"bar".repeat(n),
        })
        .collect()
}
```

<div class="mt-8 text-base opacity-80">

- `into_iter()` consumes the input, so no clone is needed.
- `match` on the enum is exhaustive: add a `Command` variant and the compiler complains here.
- `"bar".repeat(n)` replaces the inner loop. `String + &str` reuses the existing buffer.

</div>
<!--
Alternative for the `Append` arm: `let mut s = s; s.push_str(&"bar".repeat(n)); s`.
Both fine. The `s + &"bar".repeat(n)` form is shorter and reuses `s`'s allocation.
-->

---
zoom: 0.8
---

# 20 · Follow-up: commands form a pipeline

```rust
fn apply_pipeline(commands: &[Box<dyn PipelineCommand>], input: &str) -> String {
    if commands.is_empty() {
        return input.to_string();
    }

    let mut result = String::new();
    for command in commands {
        result = command.run(if result.is_empty() {
            input
        } else {
            result.as_str()
        });
    }
    result
}
```

<div class="mt-4 opacity-80">
Each command should receive the previous command's output.
</div>

<!--
The code uses an empty `String` to mean that no command has run. Empty output is valid
data. This is a sentinel bug; iterator style is beside the point.
-->

---

# 20 · Review the pipeline state

Add a command that erases its input:

```rust
struct Erase;

impl PipelineCommand for Erase {
    fn run(&self, _input: &str) -> String {
        String::new()
    }
}
```

Then test this transition:

```text
"hello" -> Erase -> "" -> Append("!") -> "!"
```

- What does the target produce instead?
- Can an empty string mean both “valid output” and “not started”?
- What value should a pipeline with no commands return?

---
zoom: 1.05
---

# 20 · Use the pipeline identity

```rust
fn apply_pipeline(
    commands: &[Box<dyn PipelineCommand>],
    input: &str,
) -> String {
    let mut current = input.to_string();
    for command in commands {
        current = command.run(&current);
    }
    current
}
```

<div class="mt-8 text-base opacity-80">

- The input is the identity value for an empty pipeline.
- Empty command output remains ordinary data.
- Initializing the real state removes both special cases.

</div>

<!--
A fold is a valid final variation, but it is not the lesson. Every output must become
the next input. This is also a useful place to compare the earlier enum dispatch with
trait-object dispatch.
-->
