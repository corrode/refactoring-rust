# `http_response_router`

> **Draft outline.** Bullet notes for the chapter; explanations still to be written.

- Review a function that chooses an application action from an HTTP status and an owned response body.
- Account for status families, exceptional statuses, text decoding, and observable output when describing the behavior to retain.
- API changes are allowed in this domain-modeling part, provided callers and tests migrate with them.

## Code to refactor

- Starter: `examples/28_http_response_router.rs`, including the complete branches omitted from the shortened slide listing.
- `Response` exposes `status: u16` and `body: Vec<u8>`, and `Action` already has `Ok(String)`, `Retry`, and `Fail(String)` variants.
- Public entry point: `handle_response(resp: Response) -> Action`, consuming the response rather than borrowing or cloning its body.
- Current approach: repeated numeric comparisons, nested checks for particular statuses, UTF-8 conversion on the success path, and branch-local `println!` calls.
- Workshop context: `slides/pages/28_http_response_router.md`, including speaker notes on separating classification from policy and using `http::StatusCode`.

## Discussion

<details class="discussion-question">
<summary>Is a status class the same thing as a routing decision?</summary>

- A 4xx classification does not determine the action by itself, because 408 and 429 retry while the other client errors fail.
- A 2xx classification still leaves body decoding and the special treatment of 204 to the handler.
- Keep factual categories separate from application policy so another consumer can use the categories without inheriting this retry behavior.
- `Action::Retry` describes a decision only, with no network request, delay, backoff, or retry limit implemented by this function.

</details>

<details class="discussion-question">
<summary>Does a 204 response bypass UTF-8 validation?</summary>

- Not in the current code: the starter converts the body before testing 204, and the solution's 204 guard runs only after conversion succeeds.
- An empty or otherwise valid UTF-8 body produces an empty success string for 204, while an invalid UTF-8 body produces `Fail("invalid utf-8")` even for 204.
- Moving the 204 check ahead of conversion would change this observable result, not merely flatten the control flow.
- Outside 2xx, the body is not decoded at all, so malformed bytes do not override the status-based action.

</details>

<details class="discussion-question">
<summary>Which guarantees would a private enum add?</summary>

- An exhaustive match requires handling every enum variant, but it does not prove that numeric ranges were classified correctly.
- `Response.status` remains a public `u16`, so callers can still construct statuses such as zero or values above 599.
- The classifier assigns those values to `Unknown` at runtime rather than rejecting response construction.
- The internal enum is a vocabulary for classification, not a validated replacement for the public response type.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Main solution: classify first, then choose an action

- Source: `solutions/28_http_response_router.rs`, retaining the public `Response`, `Action`, and `handle_response(resp: Response) -> Action` interface.
- Private `StatusClass::classify(status: u16) -> Self` maps 100–199 to `Informational`, 200–299 to `Success`, 300–399 to `Redirect`, 400–499 to `ClientError`, and 500–599 to `ServerError`.
- A final wildcard maps all remaining `u16` values to `Unknown`, making classification total without treating every value as a valid HTTP status.
- The outer match names classes using a function-local `use StatusClass::*`, while a nested success match still handles UTF-8 conversion and 204.
- `Informational` and `ServerError` retry, as do `ClientError` values 408 and 429 through a guarded arm placed before the ordinary client-error arm.
- The unguarded `ClientError` arm remains necessary because the guard does not cover the whole variant.
- Other client errors yield `client error N`, redirects yield `unhandled redirect N`, and unknown values yield `unknown status N` inside `Action::Fail`.
- Successful UTF-8 conversion yields the owned body string except for 204, which discards it and returns an empty string.
- `String::from_utf8` consumes the existing byte vector and reuses its allocation on success, so this is not a body-cloning refactor.
- Classification and action selection preserve returned actions, but all starter logging disappears rather than moving to another function.

### Alternative designs, not additional checked-in solutions

- There is no `28_http_response_router_alt.rs`; the following are design options rather than existing runnable variants.
- A direct match on raw status ranges could preserve the public API and avoid the extra enum when no other consumer needs classification.
- Keeping `StatusClass` separate is useful when several policies need the same categories, but the current method is private rather than a public classification API.
- Proposed representation change: replace `Response.status: u16` with `http::StatusCode`, constructing it through `StatusCode::from_u16` and using its class queries.
- That library type accepts 100–999, including unclassified values above 599, so it is not a guarantee that every status belongs to one of these five classes.
- A proposed structured failure type could replace `Action::Fail(String)` when callers need to inspect categories or status codes without parsing messages.
- Richer retry data would require a separate policy decision about headers, request method, retry safety, and timing, none of which the current `Response` carries.

### Caller migration and side effects

- Main solution: existing callers still construct the same public fields, move the response into `handle_response`, and match the same `Action` variants.
- Its four unit tests keep the same calls and assertions, and no external caller needs access to `StatusClass`.
- If logging is required, an outer caller can record context and act on the returned decision, but recreating the old output requires an explicit design rather than assuming it survived.
- A caller needing the status after the call must copy it before moving the response, since `Action` does not consistently retain that number.
- The proposed `StatusCode` change would require response producers and test fixtures to construct typed statuses and handle invalid numbers before invoking the otherwise unchanged handler signature.
- A structured `Action::Fail` payload would separately break callers that expect a `String`, requiring new pattern matches and an explicit message-rendering boundary.

### Edge cases, current coverage, and author decisions

- Current starter and main tests cover 200 with a valid body, retry for 503, retry for 429, and failure for 404.
- The 404 assertion checks only the variant, so even its diagnostic wording is not pinned by the current suite.
- `tests/domain_modeling.rs` has no exercise-28 fixture, classifier-exhaustiveness check, or stdout comparison.
- Proposed classifier coverage: both endpoints of every family, plus 0, 99, 600, and `u16::MAX`, with expectations for all six classes.
- Proposed handler coverage: 408, representative 1xx and 3xx responses, unknown statuses, and exact failure messages rather than variant-only assertions.
- Proposed decoding coverage: valid and invalid UTF-8 for ordinary success and 204, plus invalid bytes on non-success paths that should remain status-driven.
- Proposed compatibility coverage: compare starter and solution actions across the status domain and representative bodies, treating stdout as a separate intentionally unresolved contract.
- The slide and source comment claim a new class or special case changes exactly one place, but a new class requires the enum, classifier mapping, and exhaustive handler to agree.
- An additional exception within an existing class can stay in the handler, which is a narrower and defensible maintenance claim.
- Retrying every 1xx or 5xx response is this fixture's policy, not a universal HTTP client recommendation or a guarantee that replaying a request is safe.

<!-- TODO(prose): Decide whether removed logging is an accepted behavior change or whether a later implementation should preserve it outside the classifier, and state that choice before calling this behavior-preserving. -->
<!-- TODO(prose): Decide whether to retain decode-before-204 as the explicit exercise contract or propose a separate behavior change, and qualify the slide's exactly-one-place claim when writing the final explanation. -->

### Trade-offs

- **Type-safety:** **Original** already returns a typed `Action` but accepts any `u16` status; main adds exhaustive handling of private classes without validating public response construction or proving retry safety.
- **Readability:** **Original** interleaves repeated numeric conditions, special cases, and logging; main names the categories and localizes ranges, but readers still need the guarded 4xx arm and decode-before-204 ordering.
- **API design:** **Original** and main expose identical response and action types; a private helper avoids caller migration, while proposed typed statuses or failure payloads would change the public contract.
- **Performance:** **Original** and main both consume the body and validate UTF-8 on 2xx paths; main removes stdout work but still scans a 204 body before discarding it, with no measured dispatch speed-up claimed.
- **Flexibility:** **Original** embeds classification in one handler; main makes the categories reusable internally, though adding a class still affects its declaration, numeric mapping, and policy matches.
- **Error reporting:** **Original** offers string failures plus branch logging; main preserves returned messages but removes that output, and `Retry` still carries neither a reason nor timing information.

</details>

## Takeaways For Our Own Codebase

> - Separate a protocol category from what this application chooses to do with it.
> - Include logs and validation order in the behavior inventory before flattening branches.
> - Test boundary numbers and special cases even when enum matches are exhaustive.
> - Decide which information a returned action must retain for the caller to execute and observe it safely.

## References

- [Match expressions](https://doc.rust-lang.org/reference/expressions/match-expr.html): exhaustive coverage, range patterns, and guarded arms.
- [`String::from_utf8`](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8): fallible ownership-preserving conversion of a byte vector.
- [`http::StatusCode`](https://docs.rs/http/latest/http/status/struct.StatusCode.html): existing status representation, accepted range, and class queries.
- [Typed IDs](31-typed-ids.md): contrast an internal classification enum with a caller-visible domain type.
- [Configuration loading](29-config-loader.md): separate representation changes from deliberate failure-policy changes.
