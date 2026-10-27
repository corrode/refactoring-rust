# `mini_redis`

> **Draft outline.** Bullet notes for the chapter; explanations still to be written.

- Refactor an in-memory command interpreter for `SET key value`, `LPUSH key value [value ...]`, and `HSET key field value`.
- The first solution must preserve `handle(db: &mut Db, args: &[&str]) -> String`, the public `Db` fields and their types, and existing observable behavior.
- Keep the exercise focused on command handling, not building a Redis server.

## Code to refactor

- Source: `examples/37_mini_redis.rs`, including its six unit tests.
- Current storage: `Db` derives `Default` and exposes `strings`, `lists`, and `hashes` as separate maps.
- Current dispatch: uppercase the command, choose an `if`/`else` branch, inspect arguments, mutate the relevant map, and format a reply string.
- Callers provide an already separated argument slice, so tokenization, quoting syntax, and network input are not part of `handle`.

## Discussion

<details class="discussion-question">
<summary>Would keeping <code>handle</code> unchanged preserve the whole API?</summary>

- No, because callers can construct `Db` with a struct literal, mutate individual maps, and destructure it to take ownership of those maps.
- Replacing those fields with a single `store` breaks those callers even if `handle` keeps the same textual signature.
- The current maps also permit the same key to hold a string, list, and hash simultaneously, without any cross-type rejection.
- Removing that possibility changes behavior as well as representation, so it belongs in the alternative.

</details>

<details class="discussion-question">
<summary>What is the exact argument-count policy for each command?</summary>

- An empty argument slice returns `-ERR empty command\r\n`, while an empty command token is an unknown command.
- `SET` needs at least a key and value, returns `-ERR wrong number of arguments\r\n` when short, and ignores extras.
- `LPUSH` needs a key and at least one value, asserts when short, and consumes every supplied value when valid.
- `HSET` indexes a key, field, and value without checking first, panics when short, and ignores extras when long.
- Short `LPUSH` and `HSET` panic before a database write, so replacing those panics with error replies is a deliberate behavior change.

</details>

<details class="discussion-question">
<summary>Which list and hash results need to survive a dispatch refactor?</summary>

- `LPUSH q a b` inserts each value at index zero, leaving `b` before `a` and both ahead of any older entries.
- The list reply is its total resulting length, not the number of arguments pushed by this call.
- `HSET` replies with `:1\r\n` for a newly inserted field and `:0\r\n` for an update, even when the replacement value is unchanged.
- `SET` replies with `+OK\r\n` and replaces only the string-map entry, leaving same-key list and hash entries intact.

</details>

<details class="discussion-question">
<summary>Does uppercasing the command mean the whole input is normalized?</summary>

- Only the command token goes through Unicode-aware `to_uppercase`, while keys, fields, and values retain their original contents.
- Unknown-command replies in the original and main solution interpolate the original spelling, as in `-ERR unknown command 'froB'\r\n`.
- Empty keys, fields, and values are accepted, and whitespace is neither trimmed from tokens nor used to split them inside `handle`.
- These `&str` arguments require valid UTF-8, unlike Redis's binary-safe protocol, and quotes or physical newlines within an argument are data rather than parser syntax.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Main solution

- Source: `solutions/37_mini_redis.rs`, with the same `handle` signature, `Db::default()`, and public field layout.
- Preserve `pub strings: HashMap<String, String>` and `pub lists: HashMap<String, Vec<String>>`.
- Preserve `pub hashes: HashMap<String, HashMap<String, String>>`, including callers' direct access to inner maps.
- `split_first` handles the empty slice before dispatch, and `match` groups the three normalized command cases.
- The `SET` slice pattern `[key, value, ..]` preserves acceptance of extra arguments rather than imposing exact arity.
- `LPUSH` deliberately retains its assertion and the message `LPUSH needs key and at least one value`.
- `HSET` deliberately reads all three positional arguments before obtaining the map entry, retaining the original pre-mutation panic behavior.
- `entry(...).or_default()` preserves lazy creation of a list or hash for a missing key.
- `hash.insert(field, value).is_none()` determines whether the field was new without the original separate `contains_key` lookup.
- `u8::from(is_new)` formats the same zero-or-one result, while list order, ignored arguments, and exact error strings remain unchanged.
- No migration is needed for callers or baseline tests, and preserving undesirable behavior here does not endorse it for an untrusted network service.

### Alternative: typed values, commands, and replies

- Source: `solutions/37_mini_redis_alt.rs`, the only checked-in redesign for this exercise.
- `Db` exposes `pub store: HashMap<String, Value>` instead of the original three fields.
- Public `Value` variants contain a `String`, a `Vec<String>`, or a `HashMap<String, String>`, so each stored key has exactly one value variant.
- The store is still public, allowing callers to replace a value's variant directly rather than enforcing all updates through commands.
- `Command::parse` returns an owned `Set`, `LPush`, or `HSet` command, or `CommandError::{Empty, Unknown, WrongArity}`.
- Required fields are present in the command variants, but a publicly constructed `Command::LPush` can still contain an empty vector even though parsing rejects that input.
- Private `execute` receives a parsed command and returns a `Reply`, separating argument validation, storage operations, and output formatting.
- A missing key creates the requested variant, and an existing value of the matching variant supports the same update semantics as before.
- Each command rejects a mismatched existing variant without changing it, including `SET` on a list or hash.
- Plain Redis `SET key value` instead overwrites an existing value regardless of type, so this alternative chooses a stricter policy rather than reproducing Redis.

### API, data, and argument-policy migration

- Calls that only use `Db::default()` and `handle` can retain their syntax, but construction, field access, destructuring, and storage assertions need changes.
- Replace three-map struct literals with `Db { store: ... }` and wrap each stored value in the corresponding `Value` variant.
- Replace `db.strings.get(key)` with `db.store.get(key)` plus a `Value::String` match, and adapt list/hash reads and mutations the same way.
- Existing same-key entries in multiple maps cannot all survive a direct merge, so choose conflict rejection, an explicit precedence rule, or another migration policy rather than silently losing data.
- The alternative supplies no migration helper, and the old maps cannot reconstruct the relative write history needed for a general last-write-wins merge.
- `SET` now requires exactly two arguments after the command, and `HSET` exactly three, so previously ignored extras become errors.
- `LPUSH` still accepts one or more values, but missing arguments return an error instead of panicking.
- Every recognized wrong-arity command now returns `-ERR wrong number of arguments for 'NAME'\r\n`, including `SET` and `HSET`.
- Empty-command handling stays unchanged, while unknown-command errors now echo the uppercased spelling, so `froB` becomes `FROB`.
- Parsing errors return before mutation, and tests expecting ignored extras, original error text, or short-command panics must change only for this alternative.
- Replace the test that expects simultaneous string/list entries with an explicit cross-type policy test, while retaining the original test for the main solution.

### Reply formatting and production boundaries

- `Reply::{Ok, Integer(i64), Error(String)}` centralizes `+OK\r\n`, `:N\r\n`, and `-ERR MESSAGE\r\n` in its `Display` implementation.
- `handle` still returns a formatted `String`, so external callers do not receive a typed `Result` or a typed storage error.
- Cross-type replies are exactly `-ERR WRONGTYPE Operation against a key holding the wrong kind of value (got TYPE)\r\n`, with `TYPE` taken from the existing value.
- That reply uses the generic `ERR` prefix rather than Redis's dedicated `WRONGTYPE` prefix, even though its message contains the word `WRONGTYPE`.
- Error messages are not escaped or validated, so a command containing CR or LF can introduce additional line breaks in an unknown-command reply in every version.
- Central formatting prevents duplicated terminator code, but does not by itself make arbitrary error text safe as a protocol frame.
- The alternative casts list length from `usize` to `i64`, while the original and main format `usize` directly, without adding an explicit checked protocol conversion.
- All three implementations keep front insertion into a `Vec`, shifting existing elements for each pushed value and retaining roughly `m*n + m*m` element-movement growth for `m` pushes into `n` existing entries.
- A `VecDeque` is a possible later design, not an implemented alternative here, and changing the main solution's public list type would break compatibility.
- This is a synchronous toy interpreter, not the Tokio mini-redis server or a complete Redis implementation, and it has no sockets, request-frame parser, or partial-frame/EOF handling.
- Persistence, expiry, concurrency control, authentication, resource limits, transactions, binary payloads, and bulk/array replies are absent.
- Even familiar extensions such as `GET`, `SET` options, or multiple field/value pairs in `HSET` need additional semantics and reply support rather than a promise that one more match arm is enough.

### Existing tests and author decisions

- **Existing starter tests:** six cases cover the three writes, list reversal, hash updates, an unknown-command error prefix, and simultaneous string/list entries.
- **Existing main-solution tests:** eleven tests also cover function-pointer compatibility, struct literals, direct field mutation, destructuring, exact replies, mixed-case commands, empty data, and ignored extras.
- **Existing main-solution tests:** `short_lpush_and_hset_still_panic_without_mutating_the_database` catches each short-input panic, and a separate test checks the `LPUSH` assertion message.
- **Existing alternative tests:** six cases cover the three writes, list reversal, hash updates, an unknown-command prefix, and rejection of `SET` on an existing list.
- The alternative's tests do not establish all arity boundaries, exact error text, or every cross-type combination, even though the code contains those branches.
- **Recommended coverage:** each command at too-short, exact, and extra arities, with before/after state checks for errors and all mismatched value variants.
- **Recommended coverage:** direct `Command::parse` and `Reply::Display` cases, exact CRLF bytes, mixed-case unknown commands, and CR/LF-containing error text.
- **Recommended coverage:** repeated list pushes with existing contents, unchanged-value hash updates, Unicode command normalization, and empty keys/fields/values in the alternative.
- **Recommended compatibility checks:** execute identical command sequences against original and main databases and compare replies, all three maps, and deliberate panic behavior.
- No new tests accompany this outline, and protocol conformance is not established by the current unit suites.

<!-- TODO(prose): The slide notes and copied same-key test comment call cross-type coexistence a bug to remove, while the final slide and main solution require preserving public fields and behavior; decide how to distinguish compatibility work from the redesign in the finished narrative. -->
<!-- TODO(prose): The alternative's opening comment says rejecting SET on a list matches real Redis, but plain Redis SET replaces other types; decide whether to teach the restrictive policy as intentional or discuss a separate future Redis-compatible variant without misdescribing the current code. -->
<!-- TODO(prose): The slides frame a missing CRLF as a client hang, but the exercise has no client or transport; present this as a potential protocol integration failure rather than an observed failure in this interpreter. -->

### Trade-offs

- **Type-safety:** Original permits one key in three public maps; main preserves that model, while the alternative enforces one value variant per key but still checks command/type compatibility at runtime and exposes direct store mutation.
- **Readability:** Original keeps validation, mutation, and formatting together; main clarifies dispatch with little added machinery, while the alternative adds value, command, error, and reply enums alongside parsing, execution, and formatting.
- **API design:** Original and main support existing field-level callers; the alternative retains the handler signature but breaks the storage API, requires collision-aware data migration, and changes accepted command inputs.
- **Performance:** Original repeats a hash-field lookup that main removes; the alternative owns parsed arguments even before cross-type rejection, retains front-insertion costs and its separate hash-field lookup, and has no measured overall speed or memory advantage.
- **Flexibility:** Original and main can extend independent maps without cross-type policy; the alternative offers distinct places for new commands, values, and replies, but realistic commands can require changes to all three and additional protocol rules.
- **Error reporting:** Original and main mix error strings with deliberate arity panics; the alternative returns command-specific arity and cross-type errors, but exposes them as strings with changed spelling and a generic `ERR` prefix rather than typed caller-facing failures.

</details>

## Takeaways For Our Own Codebase

> - Include public fields, construction patterns, and direct mutations when listing the API a refactor must preserve.
> - Write down arity, cross-type, and mutation-on-error policies before centralizing command handling.
> - Separate representation invariants from command policy, since one stored value does not dictate whether a new type should replace or reject it.
> - Test exact serialized replies separately from storage behavior, and do not mistake a reply enum for a complete protocol implementation.

## References

- [Slice and rest patterns](https://doc.rust-lang.org/reference/patterns.html#slice-patterns) describe exact argument matching and patterns that accept remaining arguments.
- [`HashMap::entry`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry) provides access to occupied or vacant entries.
- [`HashMap::insert`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.insert) returns the previous value, which the main solution uses to detect a new field.
- [`Vec::insert`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.insert) documents the element shifting behind front insertion.
- [`fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) supplies the alternative's shared reply formatter.
- [Redis `SET`](https://redis.io/docs/latest/commands/set/) documents replacement of an existing value regardless of type for the plain command.
- [RESP simple errors](https://redis.io/docs/latest/develop/reference/protocol-spec/#simple-errors) distinguish the error prefix from the remaining message.
- [The six trade-off axes](../appendix/trade-off-axes.md#the-six-axes) provide the criteria for later comparison tables without an overall winner.
- [`env_file_parser` solutions](36-env-file-parser.md#solutions) separate a contract-preserving refactor from richer parsing and changed errors.
