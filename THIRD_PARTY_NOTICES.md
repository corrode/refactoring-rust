# Third-party notices for exercises 29–32

This file records the upstream sources and MIT notices for these four exercises only; it is not a repository-wide dependency inventory.
The notices below were fetched from the same commits cited in the chapters.
Keep the applicable copyright and permission notice when redistributing adapted code, including listings reproduced in the book or slides.
These notices do not change the repository's separate code and prose licenses.

## 29 · hyper buffer setters

- Project: [hyper](https://github.com/hyperium/hyper).
- Pinned source: [`src/client/conn.rs`, lines 616–642](https://github.com/hyperium/hyper/blob/73bff4e98c372ce04b006370c0b0d2af29ea8718/src/client/conn.rs#L616-L642).
- License: [MIT notice at the same commit](https://github.com/hyperium/hyper/blob/73bff4e98c372ce04b006370c0b0d2af29ea8718/LICENSE).
- Local files: `examples/29_buffer_policy.rs`, `solutions/29_buffer_policy.rs`, `book/src/part-5-domain-modeling/29-buffer-policy.md`, and `slides/pages/29_buffer_policy.md`.

The starter adapts the historical setters into a standalone configuration type, exposes fields for teaching, shortens names, removes builder chaining and feature gates, and substitutes a literal minimum and a different panic message.
The enum solution is a workshop redesign of that adaptation, not an upstream refactor or bug fix.
No upstream refactoring PR is claimed.

```text
Copyright (c) 2014-2021 Sean McArthur

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
```

## 30 · Nushell typed IDs

- Project: [Nushell](https://github.com/nushell/nushell).
- Verified PR: [#13832, Replace raw usize IDs with new types](https://github.com/nushell/nushell/pull/13832).
- Pinned source: [`crates/nu-protocol/src/id.rs`](https://github.com/nushell/nushell/blob/f0c83a445977c894c4c46ca9e1c97e3372eddd69/crates/nu-protocol/src/id.rs), at the PR's merge commit.
- License: [MIT notice at the same commit](https://github.com/nushell/nushell/blob/f0c83a445977c894c4c46ca9e1c97e3372eddd69/LICENSE).
- Local files: `examples/30_typed_ids.rs`, `solutions/30_typed_ids.rs`, `book/src/part-5-domain-modeling/30-typed-ids.md`, and `slides/pages/30_typed_ids.md`.

The `Engine` fixture and separate tuple newtypes are instructor-created examples, not excerpts from Nushell.
Upstream uses a generic `Id<T>` with marker types and `PhantomData<T>`.
The notice is recorded for the cited design source without implying that the workshop fixture is historical upstream code.

```text
MIT License

Copyright (c) 2019 - 2023 The Nushell Project Developers

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## 31 · secrecy secret access and redaction

- Project: [iqlusioninc/crates, secrecy](https://github.com/iqlusioninc/crates/tree/main/secrecy).
- Verified PR: [#1140, secrecy: rework the `SecretBox` type](https://github.com/iqlusioninc/crates/pull/1140).
- Pinned source: [`secrecy/src/lib.rs`, lines 141–162](https://github.com/iqlusioninc/crates/blob/f98d4ccf1e81d2fb9daaa7a5458f8064ecb563a2/secrecy/src/lib.rs#L141-L162), at the PR's merge commit.
- License notice recorded here: [MIT notice at the same commit](https://github.com/iqlusioninc/crates/blob/f98d4ccf1e81d2fb9daaa7a5458f8064ecb563a2/secrecy/LICENSE-MIT).
- Local files: `examples/31_secret_token.rs`, `solutions/31_secret_token.rs`, `book/src/part-5-domain-modeling/31-secret-token.md`, and `slides/pages/31_secret_token.md`.

Both workshop implementations are instructor-created fixtures inspired by explicit access and redacted debugging, not historical upstream code.
PR #1140 restructures `SecretBox`; it is not evidence of an upstream version of the starter's derived-`Debug` leak.
The toy wrapper does not reproduce secrecy's memory-handling facilities.

```text
MIT License

Copyright (c) 2019 iqlusion

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## 32 · rustls staged configuration

- Project: [rustls](https://github.com/rustls/rustls).
- Pinned source: [`rustls/src/client/builder.rs`, lines 16–31](https://github.com/rustls/rustls/blob/792045ba685965edc6f872cb8fcdd3136c8f0ef8/rustls/src/client/builder.rs#L16-L31).
- License notice recorded here: [MIT notice at the same commit](https://github.com/rustls/rustls/blob/792045ba685965edc6f872cb8fcdd3136c8f0ef8/LICENSE-MIT).
- Local files: `examples/32_config_typestate.rs`, `solutions/32_config_typestate.rs`, `book/src/part-5-domain-modeling/32-config-typestate.md`, and `slides/pages/32_config_typestate.md`.

The runtime starter and concrete-state solution are instructor-created fixtures, not upstream excerpts.
They simplify the idea demonstrated by rustls's generic typed transition and omit TLS validation and subsequent configuration stages.
The source is evidence of a typed API, not a claimed before-and-after refactoring PR.

```text
Copyright (c) 2016 Joseph Birr-Pixton <jpixton@gmail.com>

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
```
