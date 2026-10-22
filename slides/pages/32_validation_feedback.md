---
layout: default
zoom: 0.75
---

# 32 · `validation_feedback`

```rust
pub struct ValidationReport {
    pub feedback: Vec<String>,
}

impl ValidationReport {
    pub fn suggestions(&self) -> Vec<String> {
        let mut suggestions = Vec::new();
        for feedback in &self.feedback {
            match feedback.as_str() {
                "too short" => suggestions.push("Use at least eight characters.".to_owned()),
                "missing uppercase" => suggestions.push("Include an uppercase letter.".to_owned()),
                "missing digit" => suggestions.push("Include a digit.".to_owned()),
                _ => {}
            }
        }
        suggestions
    }
}
```

Keep known messages, order, and duplicates. API changes are allowed.

<div class="absolute bottom-4 left-12 right-12 text-xs opacity-70">
Fresh teaching fixture inspired by a pattern in beginner-course submissions; not verbatim learner code.
</div>

<!--
Run cargo test --example 32_validation_feedback.
The complete starter and tests are in examples/32_validation_feedback.rs.
This is feedback representation, not password security, validation rules, or scoring.
-->

---

# 32 · Review

- Which strings select a category? Which strings are for display?
- What happens to an unrecognized category?
- If you add a category, what reminds you to add its message?
- Which callers and tests would an API change affect?

```rust
let report = ValidationReport {
    feedback: vec![
        "missing digit".into(),
        "missing digits".into(),
        "missing digit".into(),
    ],
};
```

<!--
With the starter in scope, this produces two identical suggestions.
The middle entry is silently skipped. The final duplicate is preserved.
Discuss the distinction between known-category behaviour and arbitrary string input.
-->

---
zoom: 0.85
---

# 32 · Possible solution: categories

```rust
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Feedback {
    TooShort,
    MissingUppercase,
    MissingDigit,
}

impl Feedback {
    pub fn suggestion(&self) -> &'static str {
        match self {
            Self::TooShort => "Use at least eight characters.",
            Self::MissingUppercase => "Include an uppercase letter.",
            Self::MissingDigit => "Include a digit.",
        }
    }
}
```

No `_` arm: adding a variant makes this match incomplete until it has a message.

<!--
The compiler checks coverage, not whether the chosen message is correct.
Keep the exact-message tests. Copy and Clone support reuse; Debug and equality
support inspection and comparisons. Categories are distinct from display wording.
-->

---
zoom: 0.85
---

# 32 · Migrate the report and callers

```rust
pub struct ValidationReport {
    pub feedback: Vec<Feedback>,
}

impl ValidationReport {
    pub fn suggestions(&self) -> Vec<&'static str> {
        self.feedback.iter().map(Feedback::suggestion).collect()
    }
}

let report = ValidationReport {
    feedback: vec![Feedback::MissingDigit, Feedback::TooShort],
};
let messages: Vec<&'static str> = report.suggestions();
```

- Replace string keys with variants, including in helpers and tests.
- Keep known messages, order, duplicates, and non-mutation.
- Need `Vec<String>`? Convert with `.into_iter().map(str::to_owned).collect()`.

<!--
The let statements are a caller fragment, placed in a function with the types in scope.
Full implementation: solutions/32_validation_feedback.rs.
Run cargo test --example 32_validation_feedback_solution.
Static message references can outlive the report. The API is deliberately different.
The owned-string adaptation applies to report.suggestions() and allocates per message.
-->

---

# 32 · What changed, what didn't

- **Type-safety:** known categories; exhaustive message selection.
- **Readability:** category identity is separate from display wording.
- **API design:** callers migrate to `Feedback` and `Vec<&'static str>`.
- **Performance:** the result vector still allocates; static messages don't allocate individually.
- **Flexibility:** a closed set, extended through code changes.
- **Error reporting:** missing match arms become compiler errors, not skipped messages.

Wording can change without changing category-based logic **if consumers don't parse the text**.

Unknown external strings still need boundary parsing and an explicit error policy.
The enum does **not** preserve the old silent-drop contract for arbitrary strings.

<!--
Compare these six criteria against the original; this is not a drop-in replacement.
Exact-message tests and display snapshots still need updates for wording changes.
A future TryFrom<&str> implementation could return Result<Feedback, ParseFeedbackError>.
No external parser is implemented here. Do not quietly discard its errors with filter_map.
Fewer allocations are not a measured speed-up, and this is not zero allocation.
References: https://doc.rust-lang.org/book/ch06-02-match.html#matches-are-exhaustive
https://doc.rust-lang.org/std/convert/trait.TryFrom.html
-->
