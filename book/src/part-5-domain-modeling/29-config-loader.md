# `config_loader`

> **Draft outline.** Bullet notes for the chapter; explanations still to be written.

- Review access to `PORT`, `HOST`, `DEBUG`, and `MAX_CONNECTIONS` supplied through a `HashMap<String, String>`.
- Account for defaults, caller ownership, and the public `settings::Settings` interface when comparing designs.
- API redesign is allowed in this domain-modeling part, with caller migration and changes to input policy identified separately.

## Code to refactor

- Starter: `examples/29_config_loader.rs`, with four unit tests and a public `settings` module.
- `Settings` stores a public `port: u16`, alongside `new(port: u16) -> Self` and `get_port(&self) -> u16`.
- Four independent free functions borrow the map and return a port, owned host string, debug flag, or connection limit.
- Numeric getters parse text on each call, the host getter clones a string, and the debug getter compares exact spellings.
- Workshop context: `slides/pages/29_config_loader.md`, whose visibility interlude precedes the getter cleanup and typed-settings alternatives.

## Discussion

<details class="discussion-question">
<summary>Are a missing number and a malformed number the same configuration event?</summary>

- The starter treats both as defaults: 8080 for `PORT` and 100 for `MAX_CONNECTIONS`.
- `invalid_value_silently_defaults` explicitly asserts that `PORT=banana` yields 8080, so rejecting that value changes an established policy rather than fixing an accidental panic.
- Missing values can reasonably use defaults while malformed supplied values produce errors, but that distinction requires a different return contract.
- The original `unwrap_or` arguments eagerly construct default strings even when a key exists, which is a separate implementation issue from the fallback policy.

</details>

<details class="discussion-question">
<summary>Does a public getter protect a public field?</summary>

- No: callers can read or mutate `settings.port` directly and construct `Settings { port: 80 }` without using either method.
- Making the field private breaks those calls, while renaming `get_port()` to `port()` independently breaks method calls.
- Privacy would let the module control construction and mutation, but a constructor accepting any `u16` would still accept zero.
- `pub`, `pub(crate)`, and `pub(super)` select different audiences for an interface, not different levels of numeric validation.

</details>

<details class="discussion-question">
<summary>Is parsing a settings snapshot equivalent to calling the getters?</summary>

- Repeated getters consult the map each time, whereas an owned settings value records one conversion and does not follow later map changes.
- Whole-settings conversion can fail on `MAX_CONNECTIONS` even if a caller only wanted `PORT`, unlike the independent getters.
- A snapshot can avoid repeated lookups and parsing, but eager validation, reload behavior, and error timing become explicit design choices.
- There is only one input map here, with no file loading, environment access, source layering, or precedence rules implemented.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Main solution: preserve the API and fallback policy

- Source: `solutions/29_config_loader.rs`, not the typed-settings alternative suggested by some of the earlier slide prompts.
- Both numeric getters use `get(...).and_then(|value| value.parse().ok()).unwrap_or(default)`, preserving the collapse of missing and malformed values into a numeric default.
- Missing numeric keys no longer require constructing and parsing a temporary default string, and present keys no longer allocate an unused default string.
- `get_host` uses `cloned().unwrap_or_else(...)`, still returning an independent `String` but constructing the fallback only when needed.
- `get_debug` uses `matches!` on borrowed string slices, preserving exactly `"1"`, `"true"`, and `"yes"` as truthy values.
- `settings::Settings`, its public field, constructor, and `get_port` method remain unchanged, despite the permission to redesign APIs in this part.

### Checked-in alternative: typed settings with private fields

- Source: `solutions/29_config_loader_alt.rs`, combining the slides' private-representation interlude with their parse-at-the-boundary design.
- `settings::Settings` stays public but stores private `port: u16`, `host: String`, `debug: bool`, and `max_connections: u32` fields.
- `Settings::new(port: u16) -> Self` remains available and supplies localhost, false, and 100 for the other fields, without validating the port beyond its type.
- `TryFrom<&HashMap<String, String>> for Settings` has `Error = ConfigError` and returns `Result<Self, Self::Error>` from `try_from`.
- Private `parsed<T: FromStr>(map: &HashMap<String, String>, key: &'static str, default: &str) -> Result<T, ConfigError>` handles each numeric field, using defaults only for absent keys.
- `ConfigError::BadValue { key: &'static str, value: String }` retains the key and supplied text but discards the underlying parser error.
- `PORT` is parsed before `MAX_CONNECTIONS`, and `?` returns the first failure rather than collecting errors from all fields.
- The host is cloned into owned storage and debug uses the same truthy spellings, so `Settings` borrows nothing from the map after conversion.
- Privacy prevents direct field access outside `settings` and its descendants, while conversion checks text at runtime and typed fields prevent treating stored numbers as raw strings.
- The slide-only, one-field private `Settings` is a narrower design option, not another checked-in alternative file.

### Caller migration and tests that must change

- Main solution keeps `get_port(&HashMap<String, String>) -> u16`, `get_host(&HashMap<String, String>) -> String`, `get_debug(&HashMap<String, String>) -> bool`, and `get_max_connections(&HashMap<String, String>) -> u32`.
- Main callers can also keep `Settings::new`, `Settings::get_port`, struct literals, and direct field mutation, all exercised by its compatibility test.
- Alternative callers replace independent map reads with `Settings::try_from(&map)`, explicitly propagating, reporting, or handling `ConfigError` before accessing settings.
- Replace free getter calls with `port(&self) -> u16`, `host(&self) -> &str`, `debug(&self) -> bool`, and `max_connections(&self) -> u32` methods on the converted value.
- `host()` now borrows from `Settings`, so callers needing an independently owned or longer-lived host must explicitly clone or otherwise retain the settings value.
- Existing `settings.get_port()` becomes `settings.port()`, while direct field reads, assignments, and struct literals no longer compile outside the module.
- There is no replacement setter, so mutation users must deliberately rebuild from input or design an update operation rather than assuming the alternative preserves their workflow.
- The alternative replaces the starter's malformed-port fallback assertion with an error assertion, which documents a policy change rather than compatibility with the original test.

### Edge cases, current coverage, and author decisions

- The starter's four tests cover its method, three missing-key defaults, malformed `PORT` fallback, and selected debug spellings, without exercising `MAX_CONNECTIONS`.
- The main solution's seven tests add public-field mutation and literals, all four defaults, configured values, invalid numeric input, numeric boundaries, host ownership, and exact debug spellings.
- Main tests establish acceptance of zero and leading `+`, rejection-to-default of whitespace, negatives, fractions, and overflow, plus preservation of empty, padded, and Unicode host strings.
- The alternative's four tests cover its port accessor, all defaults, an invalid-port error key, and debug values `1`, `true`, and `no`, not comprehensive rejection behavior.
- Proposed alternative coverage: bad `MAX_CONNECTIONS`, numeric boundaries, first-error ordering when both numbers are bad, and the error's captured `value` rather than just its key.
- Proposed alternative coverage: snapshot independence after map mutation, borrowed host access, and compile-fail fixtures for private field access, construction, and removed methods.
- `tests/domain_modeling.rs` currently has no exercise-29 fixtures, so the proposed visibility checks are not existing integration coverage.
- Typed settings still accept zero for both numeric fields and any host string, treat unknown debug text as false, and ignore unknown keys, with no cross-field or hostname validation.
- `ConfigError` implements `Debug` but not `Display` or `std::error::Error`, so a polished operator-facing diagnostic and an underlying parse-error chain remain future work.
- The helper calls `to_string()` before parsing even on successful input, so parse-once does not mean allocation-free conversion.

<!-- TODO(prose): Decide whether to narrow the roadmap's layered-configuration promise to single-map conversion or reserve actual source precedence for later work, without implying either solution loads or merges sources. -->
<!-- TODO(prose): Keep the slides' parse-once recommendation attached to the alternative and qualify their every-getter-reparses claim, since only numeric getters parse and the main solution deliberately preserves per-call access. -->

### Trade-offs

- **Type-safety:** **Original** and main return typed scalars but expose a mutable public port; the alternative protects its stored representation and makes numeric conversion fallible without enforcing nonzero ports, valid hosts, or semantic connection limits.
- **Readability:** **Original** repeats eager fallback construction and parsing; main shortens each getter without new concepts, while the alternative centralizes conversion but adds a settings API, a generic helper, and an error type.
- **API design:** **Original** and main preserve independent owned-returning getters and public-field access; the alternative introduces a fallible snapshot, private fields, renamed methods, and a borrowed host that callers must consciously adopt.
- **Performance:** **Original** allocates default strings eagerly and reparses on every numeric read; main removes that temporary work, while the alternative amortizes parsing and host cloning across reads but still allocates numeric text during construction.
- **Flexibility:** **Original** and main observe the current map per call and allow independent field reads; the alternative offers one coherent snapshot but needs an explicit reload/update policy and rejects the whole conversion on either numeric error.
- **Error reporting:** **Original** and main silently default invalid numbers; the alternative returns the first bad numeric key and value, yet still suppresses parser detail and accepts arbitrary debug text as false.

</details>

## Takeaways For Our Own Codebase

> - Decide separately whether defaults apply to absent values, malformed values, or both.
> - Inventory direct field users before treating privacy or accessor renaming as internal cleanup.
> - Choose between live reads and a validated snapshot based on reload and failure requirements.
> - State the invariants a settings type does not enforce, even when all its fields are private.

## References

- [Visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html): module boundaries and restricted public interfaces.
- [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html): fallible conversion from the input map into owned settings.
- [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html): the bound used by the generic numeric helper, not an implementation for `Settings` itself.
- [Port parsing](../part-2-option-result/12-parse-port.md): contrast that chapter's explicit nonzero requirement with the zero accepted here.
- [Configuration typestate](34-config-typestate.md): distinguish runtime input validation from compile-time restrictions on available operations.
