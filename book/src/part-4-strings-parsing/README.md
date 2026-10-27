# Part IV. Strings, parsing & iterators

> **Draft outline.** Notes for the part introduction; the chapter links lead to the exercises and their writing outlines.

## What this part covers

- Move from aggregating collections to interpreting text, defining iteration, and composing transformations.
- Separate a shorter implementation from a changed input policy: whitespace, casing, delimiters, and integer limits are observable behavior.
- Keep original signatures in the first solution; discuss different return types and interfaces afterward as alternatives.
- Treat the compatibility questions recorded in individual outlines as decisions to settle before writing the finished chapter.

## Exercises

- [`trim_log_line`](20-trim-log-line.md): clean up a log-line transformation and pin down which whitespace and prefixes it should preserve.
- [`parse_srt_timestamp`](21-parse-srt-timestamp.md): parse a timestamp, distinguish malformed input from arithmetic limits, and compare panic-based and fallible interfaces.
- [`iterators`](22-iterators.md): produce a numeric sequence, expose it through the standard iterator protocol, and define exhaustion at the integer boundary.
- [`transformer`](23-transformer.md): review batch transformations and command pipelines, including the meaning of an empty intermediate string.
- [`fun_strings_ext`](24-fun-strings-ext.md): compare free functions with a fluent extension trait, while keeping ownership, allocation, and Unicode policy explicit.
- [`quicksort`](25-quicksort.md): optional bonus on expressing a recursive algorithm without hiding duplicate handling, worst-case work, or memory costs.

## Connections and boundaries

- Reuse the previous part's iterator vocabulary, but check whether a text operation needs bytes, Unicode scalar values, or a stricter grammar.
- Carry the distinction between ordinary iteration and domain policy into the next part's types and APIs.
- Do not present a custom iterator, extension trait, or recursive implementation as an automatic improvement over a loop or standard library method.
- The examples are small enough to trace by hand; production Unicode normalization, a full subtitle parser, and a production sorting implementation are separate requirements.
- Writing note: expand these connections into a short introduction rather than repeating each chapter's solution.
