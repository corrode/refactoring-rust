---
layout: default
---

# 27 · `mini_redis`

<div class="opacity-80 mb-4">
A tiny Redis-flavored store. Three commands, three data types.
</div>

```rust
#[derive(Default)]
pub struct Db {
    pub strings: HashMap<String, String>,
    pub lists:   HashMap<String, Vec<String>>,
    pub hashes:  HashMap<String, HashMap<String, String>>,
}

pub fn handle(db: &mut Db, args: &[&str]) -> String {
    if args.is_empty() { return "-ERR empty command\r\n".into(); }
    let cmd = args[0].to_uppercase();
    if cmd == "SET" {
        // ...
    } else if cmd == "LPUSH" {
        // ...
    } else if cmd == "HSET" {
        // ...
    }
    format!("-ERR unknown command '{}'\r\n", args[0])
}
```

<div class="absolute top-30 right-12 text-sm opacity-60">
<code>cargo test --example 27_mini_redis</code>
</div>

<!--
This is the capstone. ~3 commands now. In a real implementation:
GET, DEL, EXISTS, INCR, TYPE, LRANGE, LLEN, HGET, HKEYS, HDEL, EXPIRE,
plus 20 more. Each one slots into the if/else cascade the same way.
Imagine the file at 10x size.

Bugs to point out before they're solved:
  - Three sidecar maps mean the SAME KEY can be a string AND a list AND
    a hash simultaneously. SET clobbers nothing because there's nothing
    to clobber - the list version lives in `db.lists`. There is no
    "wrong type" error because the model has no concept of type.
  - Three different arity-check styles in one function: early return,
    `assert!`, and "panic on missing index".
  - Wire bytes (`+OK\r\n`, `:1\r\n`, `-ERR ...\r\n`) hand-written at
    every return site. One missing `\r\n` and a client hangs forever.
  - If/else on uppercased string. Adding GET means adding another
    branch and another set of mistakes.

Frame the refactor as "what's the type of a value? what's the type of
a command? what's the type of a response?" Each answer collapses code.
-->

---

# 27 · Observations

- Three storage maps for three data types. What does a *key* actually point to?
- Can the same key live in two maps at once? Should it? What would Redis itself say?
- The dispatcher uppercases a string and runs an `if/else` ladder. Where have we seen *that* antipattern in this workshop?
- `args[3]` panics when missing. The other two commands handle the same shape three different ways. Whose job is arity?
- `+OK\r\n` / `:1\r\n` / `-ERR ...\r\n` are written by hand. What is the *type* of a reply?
- Adding `GET`, `DEL`, `INCR`, `TYPE`, `LRANGE`, ... copies this whole structure for each. How many bugs scale with command count?

---

# 27 · Show me the code! 

Come up with your own solution!
Take it as far as you'd like.
