//! Read configuration values out of a `HashMap<String, String>` into typed settings.
//!
//! Solution highlights:
//! - Parsing happens once at the edge.
//! - Bad input produces a real `ConfigError` instead of a silent default.
//! - The parsing getters disappear, and callers use a small settings API.
//! - `Settings` is public while its fields remain private implementation details.

use std::collections::HashMap;

#[derive(Debug)]
pub enum ConfigError {
    BadValue { key: &'static str, value: String },
}

pub mod settings {
    use super::{ConfigError, HashMap, parsed};

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

        pub fn port(&self) -> u16 {
            self.port
        }

        pub fn host(&self) -> &str {
            &self.host
        }

        pub fn debug(&self) -> bool {
            self.debug
        }

        pub fn max_connections(&self) -> u32 {
            self.max_connections
        }
    }

    impl TryFrom<&HashMap<String, String>> for Settings {
        type Error = ConfigError;

        fn try_from(map: &HashMap<String, String>) -> Result<Self, Self::Error> {
            Ok(Self {
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
    use settings::Settings;

    #[test]
    fn settings_exposes_its_port() {
        let settings = Settings::new(8080);
        assert_eq!(settings.port(), 8080);
    }

    fn cfg(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
            .collect()
    }

    #[test]
    fn defaults_when_missing() {
        let settings = Settings::try_from(&cfg(&[])).unwrap();
        assert_eq!(settings.port(), 8080);
        assert_eq!(settings.host(), "localhost");
        assert!(!settings.debug());
        assert_eq!(settings.max_connections(), 100);
    }

    #[test]
    fn invalid_value_is_an_error() {
        let result = Settings::try_from(&cfg(&[("PORT", "banana")]));
        assert!(matches!(
            result,
            Err(ConfigError::BadValue { key: "PORT", .. })
        ));
    }

    #[test]
    fn debug_truthy_values() {
        assert!(Settings::try_from(&cfg(&[("DEBUG", "1")])).unwrap().debug());
        assert!(
            Settings::try_from(&cfg(&[("DEBUG", "true")]))
                .unwrap()
                .debug()
        );
        assert!(
            !Settings::try_from(&cfg(&[("DEBUG", "no")]))
                .unwrap()
                .debug()
        );
    }
}
