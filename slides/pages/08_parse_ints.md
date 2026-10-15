---
layout: default
---

# 08 · `parse_values`

<div class="opacity-80 mb-4">
Parse every string as <code>i32</code>. Return all parsed integers, or the first error.
</div>

```rust
pub fn parse_values(values: Vec<String>) -> Result<Vec<i32>, ParseIntError> {
    let mut result = Vec::new();
    for value in values {
        match value.parse::<i32>() {
            Ok(num) => result.push(num),
            Err(e) => return Err(e),
        }
    }
    Ok(result)
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 08_parse_ints</code>
</div>

<!--
The trick everyone enjoys discovering:
`Iterator<Item = Result<T, E>>` can be `collect`ed into
`Result<Vec<T>, E>` and returns as soon as parsing fails.
Also: `Vec<String>` -> `&[String]` (or `&[impl AsRef<str>]`) is a fair callout
but the headline is `collect`.
-->

---

# 08 · Review

<v-clicks at="+0">

- What does `collect` do when the items are `Result`s?
- Does the function need to *own* the `Vec<String>` to read from it?
- The early return on `Err` is doing the same thing `collect` does for free.

</v-clicks>

---
zoom: 1.2
---

# 08 · Possible solution

```rust
pub fn parse_values(values: &[String]) -> Result<Vec<i32>, ParseIntError> {
    values.iter().map(|v| v.parse()).collect()
}
```

<div v-click class="mt-8 text-base opacity-80">

- `collect` is overloaded: from `Iterator<Item = Result<T, E>>` it builds `Result<Vec<T>, E>` and stops at the first `Err`.
- The return type drives type inference for `parse` and `collect`.
- Borrowed slice means callers don't have to give up ownership.

</div>

---
zoom: 0.9
---

# 08 · Same iterator, different policy

<div class="opacity-80 mb-3">
Validation wants every rule to run and every error to be returned in rule order.
</div>

```rust
fn collect_errors(
    validators: &[&dyn Validator],
    input: &str,
) -> Vec<String> {
    let mut errors = Vec::new();
    for validator in validators {
        let result = validator.check(input);
        if result.is_err() {
            errors.push(
                result.err().expect("is_err should have an error")
            );
        }
    }
    errors
}
```

<!--
This follows parse_values deliberately. Ask whether the previous collect::<Result>
refactor should be reused. It should not: later validators would never run after an error.
-->

---

# 08 · Review the error policy

- Must every validator run, even after the first error?
- Must errors preserve validator order?
- Why does the target ask the same `Result` about `is_err`, `err`, and `expect`?
- What would `collect::<Result<Vec<_>, _>>()` do here?

<div class="mt-12 opacity-70">
Shorter code still has to preserve collect-all behavior.
</div>

---
zoom: 1.1
---

# 08 · Collect every validation error

```rust
fn collect_errors(
    validators: &[&dyn Validator],
    input: &str,
) -> Vec<String> {
    validators
        .iter()
        .filter_map(|validator| validator.check(input).err())
        .collect()
}
```

<div class="mt-8 text-base opacity-80">

- `Result::err` turns successful checks into `None` and failures into `Some(error)`.
- `filter_map` visits every validator and keeps every error.
- Parsing still uses `collect::<Result<_, _>>()` because an error should end the operation.

</div>
