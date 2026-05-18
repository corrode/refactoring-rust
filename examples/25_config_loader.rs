//! Read configuration values out of a `HashMap<String, String>`.

use std::collections::HashMap;

pub fn get_port(config: &HashMap<String, String>) -> u16 {
    config
        .get("PORT")
        .unwrap_or(&"8080".to_string())
        .parse()
        .unwrap_or(8080)
}

pub fn get_host(config: &HashMap<String, String>) -> String {
    config
        .get("HOST")
        .unwrap_or(&"localhost".to_string())
        .clone()
}

pub fn get_debug(config: &HashMap<String, String>) -> bool {
    match config.get("DEBUG") {
        Some(v) => v == "1" || v == "true" || v == "yes",
        None => false,
    }
}

pub fn get_max_connections(config: &HashMap<String, String>) -> u32 {
    config
        .get("MAX_CONNECTIONS")
        .unwrap_or(&"100".to_string())
        .parse()
        .unwrap_or(100)
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
        let c = cfg(&[]);
        assert_eq!(get_port(&c), 8080);
        assert_eq!(get_host(&c), "localhost");
        assert!(!get_debug(&c));
    }

    #[test]
    fn invalid_value_silently_defaults() {
        let c = cfg(&[("PORT", "banana")]);
        assert_eq!(get_port(&c), 8080);
    }

    #[test]
    fn debug_truthy_values() {
        assert!(get_debug(&cfg(&[("DEBUG", "1")])));
        assert!(get_debug(&cfg(&[("DEBUG", "true")])));
        assert!(!get_debug(&cfg(&[("DEBUG", "no")])));
    }
}
