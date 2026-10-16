---
layout: default
---

# 26 · `Settings`

<div class="opacity-80 mb-4">
Make a settings type available to callers without exposing more than they need.
</div>

```rust
pub mod settings {
    pub struct Settings {
        pub port: u16,
    }

    impl Settings {
        pub fn new(port: u16) -> Self {
            Self { port }
        }

        pub fn get_port(&self) -> u16 {
            self.port
        }
    }
}
```

<!--
The type, constructor, accessor, and field are all public. The code compiles, but the
field makes the accessor redundant and commits the module to this representation.
-->

---

# 26 · Review the public surface

- Callers need to construct `Settings` and read the port. Do they need direct field access?
- What future changes become breaking changes once the field is public?
- Which items need `pub`, and which can remain private?
- When would `pub(crate)` or `pub(super)` describe the intended boundary better?

<div class="mt-12 opacity-70">
Fix visibility errors one at a time with the narrowest access that serves the caller.
</div>

---

# 26 · Keep the representation private

```rust
pub mod settings {
    pub struct Settings {
        port: u16,
    }

    impl Settings {
        pub fn new(port: u16) -> Self {
            Self { port }
        }

        pub fn port(&self) -> u16 {
            self.port
        }
    }
}
```

<div class="mt-8 text-base opacity-80">

- Callers depend on the type's API rather than its field layout.
- The module can add validation or change storage without changing callers.
- Visibility defines an architectural boundary.

</div>

---
layout: default
zoom: 0.85
---

# 26 · `config_loader`

<div class="opacity-80 mb-4">
Read configuration values out of a <code>HashMap&lt;String, String&gt;</code>.
</div>

```rust
use std::collections::HashMap;

pub fn get_port(config: &HashMap<String, String>) -> u16 {
    config.get("PORT").unwrap_or(&"8080".to_string())
        .parse().unwrap_or(8080)
}

pub fn get_host(config: &HashMap<String, String>) -> String {
    config.get("HOST").unwrap_or(&"localhost".to_string()).clone()
}

pub fn get_debug(config: &HashMap<String, String>) -> bool {
    match config.get("DEBUG") {
        Some(v) => v == "1" || v == "true" || v == "yes",
        None => false,
    }
}

pub fn get_max_connections(config: &HashMap<String, String>) -> u32 {
    config.get("MAX_CONNECTIONS").unwrap_or(&"100".to_string())
        .parse().unwrap_or(100)
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 26_config_loader</code>
</div>

<!--
Slightly compressed whitespace to fit four functions on one slide.

Discoveries:
  - Every getter re-parses the map. State and parsing are intertwined.
  - Errors are swallowed silently (`"banana"` for MAX_CONNECTIONS → 100).
  - A typed settings struct is hiding inside the string-to-string map.
  - Parse once into that struct. The rest of the program can use typed values.
-->

---

# 26 · Review

- Each getter re-parses the same map. Should this happen once, or per call?
- <code>"banana"</code> for <code>MAX_CONNECTIONS</code> silently becomes <code>100</code>. Is that what an operator wants?
- The map is stringly-typed; the program wants a real <code>Settings</code>. Where should the conversion live?
- If we parsed once into a struct, how many of these functions survive?

<div class="mt-12 opacity-70">
Hint: you can <code>impl TryFrom&lt;&HashMap&lt;String, String&gt;&gt; for Settings</code>.
</div>

---
zoom: 0.75
---

# 26 · Possible solution: public surface

```rust
pub mod settings {
    pub struct Settings {
        port: u16,
        host: String,
        debug: bool,
        max_connections: u32,
    }

    impl Settings {
        pub fn new(port: u16) -> Self {
            Self {
                port,
                host: "localhost".to_string(),
                debug: false,
                max_connections: 100,
            }
        }

        pub fn port(&self) -> u16 { self.port }
        pub fn host(&self) -> &str { &self.host }
        pub fn debug(&self) -> bool { self.debug }
        pub fn max_connections(&self) -> u32 { self.max_connections }
    }
}
```

<div class="mt-4 text-base opacity-80">
The type and its operations are public. Its representation remains private.
</div>

---
zoom: 0.72
---

# 26 · Possible solution: parse at the boundary

```rust
#[derive(Debug)]
pub enum ConfigError {
    BadValue { key: &'static str, value: String },
}

fn parsed<T: std::str::FromStr>(map: &HashMap<String, String>,
                      key: &'static str, default: &str)
    -> Result<T, ConfigError>
{
    let raw = map.get(key).map(String::as_str)
        .unwrap_or(default).to_string();
    raw.parse().map_err(|_| ConfigError::BadValue { key, value: raw })
}

// Inside `settings`, with `parsed`, `ConfigError`, and `HashMap` imported:
impl TryFrom<&HashMap<String, String>> for Settings {
    type Error = ConfigError;

    fn try_from(map: &HashMap<String, String>) -> Result<Self, Self::Error> {
        Ok(Self {
            port: parsed(map, "PORT", "8080")?,
            max_connections: parsed(map, "MAX_CONNECTIONS", "100")?,
            host: map.get("HOST").cloned()
                .unwrap_or_else(|| "localhost".into()),
            debug: matches!(map.get("DEBUG").map(String::as_str),
                Some("1" | "true" | "yes")),
        })
    }
}
```

<div class="mt-4 text-base opacity-80">
Parsing happens once, bad input is reported, and callers use the small `Settings` API.
</div>

<!--
Once you have `Settings`, you can't forget to parse a field or read `PORT`
as a string by accident. The private fields preserve the visibility decision
from the opening interlude.

In production we'd usually reach for `serde` with `envy`, `figment`, or `config-rs`.
Doing the conversion by hand shows what those crates save us from maintaining.
-->
