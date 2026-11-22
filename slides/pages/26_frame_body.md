---
layout: default
zoom: 0.95
---

# 26 · `frame_body`

```rust
pub fn frame_body(frame: &[u8]) -> Option<(u8, &[u8])> {
    if frame.len() < 3 {
        return None;
    }

    let last = frame.len() - 1;
    if frame[0] != 0xA5 || frame[last] != 0x5A {
        return None;
    }

    let kind = frame[1];
    if !(1..=3).contains(&kind) {
        return None;
    }

    Some((kind, &frame[2..last]))
}
```

<div class="absolute bottom-4 left-12 right-12 text-xs opacity-70">
Opening <code>0xA5</code>, kind <code>1..=3</code>, any payload bytes, closing <code>0x5A</code>.<br>
Keep the signature and behaviour. Full source: <code>examples/26_frame_body.rs</code>.
</div>

<!--
Run cargo test --example 26_frame_body. This starter is correct, safe, and
allocation-free. Do not invent an indexing bug or promise an allocation win.
The caller provides a complete buffer. No streaming parser or payload decoding.
The task is to make the implementation easier to follow without changing inputs.
-->

---

# 26 · Review

- What makes each index and slice boundary valid?
- Is an empty payload allowed?
- What if the payload contains `0x5A`?
- Does returning the payload copy its bytes?

<!--
The minimum length is three: marker, kind, marker. Then last >= 2, so 2..last
is valid even when empty. The closing marker must be the final byte, not the
first occurrence of 0x5A. Both marker bytes are valid payload bytes. The starter
already borrows the payload and copies only the kind byte into its result.
-->

---

# 26 · Possible solution

```rust
pub fn frame_body(frame: &[u8]) -> Option<(u8, &[u8])> {
    match frame {
        [0xA5, kind @ 1..=3, body @ .., 0x5A] => Some((*kind, body)),
        _ => None,
    }
}
```

- One pattern checks the layout and binds the needed values.
- `kind @ 1..=3` names a byte that matches the range.
- `body @ ..` borrows the middle, which may be empty.
- Both versions are allocation-free; the change removes index bookkeeping.

<div class="absolute bottom-4 left-12 right-12 text-xs opacity-70">
Full source and the same eight tests: <code>solutions/26_frame_body.rs</code>.
</div>

<!--
Match ergonomics give kind: &u8 and body: &[u8]. *kind copies a single byte.
The pattern checks the minimum length before entering the arm. Range and marker
validation remain runtime checks; the return type is still an ordinary u8.
The payload lifetime remains tied to the input. No caller migration is required.
Readability depends on familiarity. Fewer lines do not prove a speed-up.
-->

---

# 26 · The last byte, not the next marker

```rust
let packet = [0xA5, 2, 0x5A, 10, 0xA5, 0x5A];
assert_eq!(frame_body(&packet), Some((2, &[0x5A, 10, 0xA5][..])));
```

- The payload may contain either marker, and need not be text.
- The supplied buffer determines where the frame ends.
- Concatenated frames can be interpreted as one larger frame.
- Discovering boundaries in a stream needs a separate rule.

<div class="absolute bottom-4 left-12 right-12 text-xs opacity-70">
<a href="https://doc.rust-lang.org/reference/patterns.html#slice-patterns">Slice patterns</a> · <a href="https://doc.rust-lang.org/reference/patterns.html#rest-patterns">Rest patterns</a>
</div>

<!--
The rest pattern does not search for a delimiter. The entire input is matched,
with a fixed prefix and suffix and an arbitrary middle. A stream would need
length information, escaping, or another framing mechanism. Tests preserve
this whole-buffer contract, including empty payloads and internal marker bytes.
Quicksort reuses slice patterns next, this time separating a pivot and tail.
Keep owned-array binding rules in the cheatsheet rather than expanding this task.
-->
