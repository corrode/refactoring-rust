# Part II. Option, Result, `?`

`Option` represents a value that may be absent. `Result` represents success or failure
and can carry error details. The function's contract determines whether absence is
acceptable, which errors callers need, and whether a failure should stop further work.

This part covers replacing nested branches with `Option` and `Result` methods,
combining transformations, and using `?` to return early. You'll compare stopping at the
first failure with processing all inputs. Each refactor must preserve when work stops
and what the caller learns, unless changing that behavior is an explicit goal.
