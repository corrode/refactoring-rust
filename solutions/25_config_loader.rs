//! Read configuration values out of a `HashMap<String, String>` into a typed `Config`.
//!
//! Solution highlights:
//! - Parsing happens once, at the edge. The rest of the program works with a typed `Config`.
//! - Bad input produces a real `ConfigError` instead of a silent default.
//! - The four getters disappear — code reads `config.port`, `config.host`, …

use std::collections::HashMap;

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
        Ok(Config {
            port: parsed(map, "PORT", "8080")?,
            max_connections: parsed(map, "MAX_CONNECTIONS", "100")?,
            host: map
                .get("HOST")
                .cloned()
                .unwrap_or_else(|| "localhost".into()),
            debug: matches!(
                map.get("DEBUG").map(String::as_str),
                Some("1" | "true" | "yes")
            ),
        })
    }
}

fn parsed<T: std::str::FromStr>(
    map: &HashMap<String, String>,
    key: &'static str,
    default: &str,
) -> Result<T, ConfigError> {
    let raw = map
        .get(key)
        .map(String::as_str)
        .unwrap_or(default)
        .to_string();
    raw.parse()
        .map_err(|_| ConfigError::BadValue { key, value: raw })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect()
    }

    #[test]
    fn defaults_when_missing() {
        let c = Config::try_from(&cfg(&[])).unwrap();
        assert_eq!(c.port, 8080);
        assert_eq!(c.host, "localhost");
        assert!(!c.debug);
        assert_eq!(c.max_connections, 100);
    }

    #[test]
    fn invalid_value_silently_defaults() {
        // After the refactor this is no longer silent: a bad PORT is an error.
        let result = Config::try_from(&cfg(&[("PORT", "banana")]));
        assert!(matches!(
            result,
            Err(ConfigError::BadValue { key: "PORT", .. })
        ));
    }

    #[test]
    fn debug_truthy_values() {
        assert!(Config::try_from(&cfg(&[("DEBUG", "1")])).unwrap().debug);
        assert!(Config::try_from(&cfg(&[("DEBUG", "true")])).unwrap().debug);
        assert!(!Config::try_from(&cfg(&[("DEBUG", "no")])).unwrap().debug);
    }
}
