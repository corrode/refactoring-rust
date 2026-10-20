---
layout: default
zoom: 0.9
---

# 33 · `config_typestate`

```rust
#[derive(Default)]
pub struct ConfigBuilder {
    roots: Option<RootStore>,
}

impl ConfigBuilder {
    pub fn with_roots(mut self, roots: RootStore) -> Self {
        self.roots = Some(roots);
        self
    }

    pub fn build(self) -> Result<ClientConfig, &'static str> {
        let roots = self.roots.ok_or("missing roots")?;
        Ok(ClientConfig { roots })
    }
}
```

<div class="absolute bottom-6 left-12 right-12 text-xs opacity-70">
Inspired by <a href="https://github.com/rustls/rustls">rustls</a>. This runtime starter is ours, not historical rustls code.<br>
<code>RootStore</code> holds a label, not certificates. Full source: <code>examples/33_config_typestate.rs</code>.
</div>

<!--
Review the API, not certificate verification. RootStore is a non-Clone wrapper
around String; ClientConfig owns the supplied store. API changes are allowed.
The original builder permits repeated settings, keeping the last store.
-->

---

# 33 · Review

- Can callers build before supplying roots?
- What happens when callers supply roots twice?
- Which operations should each stage expose?
- Does supplying roots prove they are trustworthy?

<div class="absolute bottom-6 left-12 right-12 text-xs opacity-70">
Rust: <a href="https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html">ownership and moves</a> · <a href="https://doc.rust-lang.org/book/ch05-03-method-syntax.html">consuming receivers</a>
</div>

<!--
The starter's ownership already prevents reuse after consumption, but it does
not prevent early build. Ask whether replacement is a requirement before
removing it. Missing configuration and invalid external data are different
problems; a supplied store can still be empty or inappropriate.
-->

---
zoom: 0.9
---

# 33 · Possible solution: concrete states

```rust
#[derive(Default)]
pub struct NeedsRoots;
pub struct Ready { roots: RootStore }

impl NeedsRoots {
    pub fn with_roots(self, roots: RootStore) -> Ready {
        Ready { roots }
    }
}
impl Ready {
    pub fn build(self) -> ClientConfig {
        ClientConfig { roots: self.roots }
    }
}
```

- `NeedsRoots` cannot build; `Ready` cannot replace roots.
- Callers use `NeedsRoots::default()` and remove `build().unwrap()`.

<div class="absolute bottom-6 left-12 right-12 text-xs opacity-70">
Rust: <a href="https://doc.rust-lang.org/reference/visibility-and-privacy.html">private fields</a> restrict direct construction to the defining module and its descendants.<br>
Full source: <code>solutions/33_config_typestate.rs</code>. Neither state implements <code>Copy</code> or <code>Clone</code>.
</div>

<!--
Each transition consumes its receiver and moves the store. Ready has no public
constructor or Default implementation. Its private field restricts construction
outside the defining module and its descendants. build returns ClientConfig, not Result.
The solution changes the early-build and repeated-setting contracts. Callers
choose the final store before transitioning; replacement could be a separate
Ready method if required. Runtime tests check preserved content and allocation;
compile-fail tests should cover early build and reuse after consumption.
-->

---

# 33 · What does `Ready` prove?

- A store was supplied, not validated.
- Owning `Ready` permits one consuming build.
- A required constructor argument may be enough.
- Use runtime errors for data-dependent failures.

<div class="absolute bottom-6 left-12 right-12 text-xs opacity-70">
<a href="https://github.com/rustls/rustls/blob/792045ba685965edc6f872cb8fcdd3136c8f0ef8/rustls/src/client/builder.rs#L16-L31">Pinned rustls typed builder</a>: <code>WantsVerifier</code> → <code>WantsTransparencyPolicyOrClientCert</code>.<br>
Our concrete states simplify that generic transition. This is source evidence, not a verified refactoring PR.
</div>

<!--
Upstream uses ConfigBuilder<ClientConfig, State> and has further TLS choices.
We omit those choices, certificate parsing, verification, and trust policy.
An empty RootStore label passes both versions. Ownership is a capability, not
certification of trustworthy contents. Two concrete types avoid introducing
generic state machinery for one transition. A constructor taking roots would
be shorter here; staged builders earn their cost with dependent choices.
-->
