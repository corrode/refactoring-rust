# Part II. Option, Result, `?`

Rust puts absence and failure where you can see them: in the type system. That does not
choose the right policy for you. A nested `match` can bury a small question, an `Option`
can discard an error the caller needs, and `?` can stop work that was supposed to
continue.

In this part, you'll treat `Option` and `Result` as contracts, not syntax to get rid of.
You'll flatten nested control flow, compose small transformations, choose deliberately
between fail-fast and collect-all behavior, and keep useful errors intact. These chapters
build on the warm-ups, but the cost of a careless cleanup is higher: tidier code can still
change when work stops or what the caller learns about a failure.
