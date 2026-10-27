---
layout: default
---

# 04 · `person_info`

Describe a person using the existing age bands. Keep the signature and output unchanged.

```rust
pub struct Person {
    pub name: String,
    pub age: u8,
}

pub fn person_info(person: &Person) -> String {
    match person.age {
        0..=19 => format!("{} is a teenager.", person.name),
        20..=39 => format!("{} is an adult.", person.name),
        _ => format!("{} is middle-aged.", person.name),
    }
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 04_person_info</code>
</div>

<!--
The labels are deliberately preserved, including zero as a teenager and every
age from forty onward as middle-aged. Changing that policy is a separate task.
Both versions and their tests are in the 04_person_info Cargo targets.
-->

---

# 04 · Review

- Each arm checks an age and uses a name. Can the pattern express both?
- If we match `&Person`, what type will a bound `name` have?
- What does `..` ignore in a struct pattern?
- Does changing the pattern remove any string allocation?

---

# 04 · Possible solution

```rust
pub fn person_info(person: &Person) -> String {
    match person {
        Person { name, age: 0..=19 } => format!("{name} is a teenager."),
        Person { name, age: 20..=39 } => format!("{name} is an adult."),
        Person { name, .. } => format!("{name} is middle-aged."),
    }
}
```

- Match ergonomics bind `name` as `&String`; the person is still borrowed.
- `..` ignores other fields; earlier arms decide which ages reach the fallback.
- Both versions borrow the name and allocate the formatted result.

<!--
Field shorthand means name: name. The range is inclusive at both ends.
The first two patterns list all fields; adding a field to Person requires
updating those patterns or adding .. to ignore the new field deliberately.
The original field match is also reasonable. The goal is to expose the values
an arm uses, not to claim destructuring is always shorter or faster.
-->
