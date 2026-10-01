# Part II. Option, Result, `?`

Rust makes absence and failure visible in the type system. That helps only when the code
uses those types with care. A nested `match` can hide a small question, an `Option` can
throw away an error your caller needs, and `?` can stop work that was meant to continue.

In this part, you'll practise treating `Option` and `Result` as contracts rather than
syntax to remove. You'll flatten nested control flow, compose small transformations,
choose between fail-fast and collect-all behavior, and preserve useful errors. These
chapters build on the warm-ups, but the stakes are higher now: a tidy refactor can still
change when work stops or what a caller learns from failure.
