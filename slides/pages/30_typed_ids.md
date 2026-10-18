---
layout: default
zoom: 0.85
---

# 30 · `typed_ids`

```rust
pub type VarId = usize;
pub type DeclId = usize;

pub struct Engine {
    variables: Vec<String>,
    declarations: Vec<String>,
}

impl Engine {
    pub fn new(variables: Vec<String>, declarations: Vec<String>) -> Self {
        Self { variables, declarations }
    }
    pub fn variable(&self, id: VarId) -> Option<&str> {
        self.variables.get(id).map(String::as_str)
    }
    pub fn declaration(&self, id: DeclId) -> Option<&str> {
        self.declarations.get(id).map(String::as_str)
    }
}
```

<div class="absolute bottom-4 left-12 right-12 text-xs opacity-70">
Workshop-created fixture inspired by <a href="https://github.com/nushell/nushell">Nushell</a> · <a href="https://github.com/nushell/nushell/pull/13832">PR #13832</a>, not an engine excerpt.
</div>

<!--
Run cargo test --example 30_typed_ids. The source and tests are in
examples/30_typed_ids.rs. Review the lookup API; API changes are allowed.
Preserve borrowed results and None for missing entries.
-->

---

# 30 · Review

- Can a `DeclId` reach `variable`? What happens if the index exists?
- Does an alias create a new type?
- Where should callers choose an ID category?
- Would distinct types prove that an entry exists or belongs to this engine?

```rust
let engine = Engine::new(vec!["count".into()], vec!["print".into()]);
let variable: VarId = 0;
let declaration: DeclId = 0;
engine.variable(declaration); // Compiles with aliases.
engine.declaration(variable); // Compiles with aliases.
```

<div class="absolute bottom-4 left-12 right-12 text-xs opacity-70">
Workshop fixture · <a href="https://github.com/nushell/nushell/pull/13832">Nushell ID refactor</a> · <a href="https://github.com/nushell/nushell/blob/f0c83a445977c894c4c46ca9e1c97e3372eddd69/crates/nu-protocol/src/id.rs#L81-L87">Pinned category aliases</a>
</div>

<!--
The snippet continues the starter's definitions. Both aliases are usize.
Option catches out-of-range indices, not in-range category mistakes.
-->

---
zoom: 0.85
---

# 30 · Possible solution: distinct IDs

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VarId(pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeclId(pub usize);

impl Engine {
    pub fn variable(&self, id: VarId) -> Option<&str> {
        self.variables.get(id.0).map(String::as_str)
    }
    pub fn declaration(&self, id: DeclId) -> Option<&str> {
        self.declarations.get(id.0).map(String::as_str)
    }
}
```

- Callers construct `VarId(0)` and `DeclId(0)`; swapped calls now fail to compile.
- `Engine::new` stays unchanged. Missing entries still return `None`.
- Category safety does not guarantee index validity or engine identity.
- Nushell uses generic `Id<T>` + `PhantomData<T>`; these two structs simplify the lesson.

<div class="absolute bottom-4 left-12 right-12 text-xs opacity-70">
Teaching simplification · <a href="https://github.com/nushell/nushell/pull/13832">Nushell PR #13832</a> · <a href="https://github.com/nushell/nushell/blob/f0c83a445977c894c4c46ca9e1c97e3372eddd69/crates/nu-protocol/src/id.rs#L7-L33">Pinned generic implementation</a>
</div>

<!--
Replace the aliases and lookup methods; keep Engine's storage and constructor.
The full solution and tests are in solutions/30_typed_ids.rs.
Run cargo test --example 30_typed_ids_solution.
With the solution types in scope, engine.variable(DeclId(0)) reports expected
VarId, found DeclId; engine.declaration(VarId(0)) reports the reverse mismatch.
Public fields permit explicit relabeling. There is no claim of validation.
TypeId unit tests establish distinct types, not compile-fail regression coverage.
Check swapped calls separately with compile-fail tests.
-->
