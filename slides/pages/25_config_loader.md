---
layout: default
zoom: 0.85
---

# 25 · `config_loader`

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
<code>cargo test --example 25_config_loader</code>
</div>

<!--
Slightly compressed whitespace to fit four functions on one slide.

Discoveries:
  - Every getter re-parses the map. State and parsing are intertwined.
  - Errors are swallowed silently (`"banana"` for MAX_CONNECTIONS → 100).
  - The "config" is really a typed struct hiding inside a string→string map.
  - Parse once, into a real struct. Then the rest of the program just uses fields.
-->

---

# 25 · Observations

- Each getter re-parses the same map. Should this happen once, or per call?
- <code>"banana"</code> for <code>MAX_CONNECTIONS</code> silently becomes <code>100</code>. Is that what an operator wants?
- The map is stringly-typed; the program wants a real <code>Config</code>. Where should the conversion live?
- If we parsed once into a struct, how many of these functions survive?

<div class="mt-12 opacity-70">
Hint: you can <code>impl TryFrom&lt;&HashMap&lt;String, String&gt;&gt; for Config</code>.
</div>

---
zoom: 0.8
---

# 25 · Possible solution

```rust
pub struct Config {
    pub port: u16,
    pub host: String,
    pub debug: bool,
    pub max_connections: u32,
}

#[derive(Debug)]
pub enum ConfigError {
    BadValue { key: &'static str, value: String },
}

impl TryFrom<&HashMap<String, String>> for Config {
    type Error = ConfigError;
    fn try_from(map: &HashMap<String, String>) -> Result<Self, Self::Error> {
        let parsed = |key, default: &str| -> Result<_, _> {
            let raw = map.get(key).map(String::as_str).unwrap_or(default).to_string();
            raw.parse().map_err(|_| ConfigError::BadValue { key, value: raw })
        };
        Ok(Config {
            port: parsed("PORT", "8080")?,
            max_connections: parsed("MAX_CONNECTIONS", "100")?,
            host: map.get("HOST").cloned().unwrap_or_else(|| "localhost".into()),
            debug: matches!(map.get("DEBUG").map(String::as_str), Some("1" | "true" | "yes")),
        })
    }
}
```

<div class="mt-8 text-base opacity-80">

- Parsing happens **once**, at the edge. The rest of the program works with a typed `Config`.
- Bad input produces a real `ConfigError` instead of a silent default.
- The four getters disappear - code reads `config.port`, `config.host`, …

</div>
<!--
"Make illegal states unrepresentable" - once you have a `Config`, you can't
forget to parse a field, and you can't read `PORT` as a string by accident.

In a real codebase you'd reach for `serde` + `envy` or `figment` or `config-rs`
to do exactly this - but the point of the exercise is to see *why* those crates
exist by feeling the pain of doing it by hand, std-only.
-->
