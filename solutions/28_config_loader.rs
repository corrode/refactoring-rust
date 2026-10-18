//! Read configuration values without changing the caller's API or fallback policy.
//! See `28_config_loader_alt.rs` for typed settings with explicit parse errors.

// ANCHOR: listing
use std::collections::HashMap;

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

pub fn get_port(config: &HashMap<String, String>) -> u16 {
    config
        .get("PORT")
        .and_then(|value| value.parse().ok())
        .unwrap_or(8080)
}

pub fn get_host(config: &HashMap<String, String>) -> String {
    config
        .get("HOST")
        .cloned()
        .unwrap_or_else(|| "localhost".to_string())
}

pub fn get_debug(config: &HashMap<String, String>) -> bool {
    matches!(
        config.get("DEBUG").map(String::as_str),
        Some("1" | "true" | "yes")
    )
}

pub fn get_max_connections(config: &HashMap<String, String>) -> u32 {
    config
        .get("MAX_CONNECTIONS")
        .and_then(|value| value.parse().ok())
        .unwrap_or(100)
}
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
            .collect()
    }

    #[test]
    fn settings_preserves_constructor_getter_and_public_field() {
        let mut settings = settings::Settings::new(8080);
        assert_eq!(settings.get_port(), 8080);
        settings.port = 9000;
        assert_eq!(settings.get_port(), 9000);
        assert_eq!(settings::Settings { port: 80 }.get_port(), 80);
    }

    #[test]
    fn defaults_when_missing() {
        let config = cfg(&[]);
        assert_eq!(get_port(&config), 8080);
        assert_eq!(get_host(&config), "localhost");
        assert!(!get_debug(&config));
        assert_eq!(get_max_connections(&config), 100);
    }

    #[test]
    fn configured_values() {
        let config = cfg(&[
            ("PORT", "9000"),
            ("HOST", "example.com"),
            ("MAX_CONNECTIONS", "200"),
        ]);
        assert_eq!(get_port(&config), 9000);
        assert_eq!(get_host(&config), "example.com");
        assert_eq!(get_max_connections(&config), 200);
    }

    #[test]
    fn invalid_numbers_silently_default() {
        for value in ["banana", "", "-1", " 80", "80 ", "1.5", "4294967296"] {
            let config = cfg(&[("PORT", value), ("MAX_CONNECTIONS", value)]);
            assert_eq!(get_port(&config), 8080, "{value:?}");
            assert_eq!(get_max_connections(&config), 100, "{value:?}");
        }
        assert_eq!(get_port(&cfg(&[("PORT", "65536")])), 8080);
    }

    #[test]
    fn numeric_boundaries_and_leading_plus_are_accepted() {
        for (value, expected) in [("0", 0), ("65535", u16::MAX), ("+80", 80)] {
            assert_eq!(get_port(&cfg(&[("PORT", value)])), expected);
        }
        for (value, expected) in [("0", 0), ("4294967295", u32::MAX), ("+200", 200)] {
            assert_eq!(
                get_max_connections(&cfg(&[("MAX_CONNECTIONS", value)])),
                expected
            );
        }
    }

    #[test]
    fn host_is_owned_and_not_normalized() {
        for host in ["", "  localhost  ", "例え.test"] {
            let config = cfg(&[("HOST", host)]);
            let mut owned = get_host(&config);
            assert_eq!(owned, host);
            owned.push('!');
            assert_eq!(get_host(&config), host);
        }
    }

    #[test]
    fn debug_only_accepts_exact_truthy_values() {
        for value in ["1", "true", "yes"] {
            assert!(get_debug(&cfg(&[("DEBUG", value)])));
        }
        for value in ["", "0", "false", "no", "TRUE", "Yes", " true", "yes "] {
            assert!(!get_debug(&cfg(&[("DEBUG", value)])));
        }
    }
}
// ANCHOR_END: tests
