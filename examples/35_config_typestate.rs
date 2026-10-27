// Provenance: instructor-created runtime builder, not historical rustls code.
// Inspired by rustls's typed builder, not a claimed upstream refactoring PR.
// Project: https://github.com/rustls/rustls
// Pinned source: https://github.com/rustls/rustls/blob/792045ba685965edc6f872cb8fcdd3136c8f0ef8/rustls/src/client/builder.rs#L16-L31
// Upstream MIT notice: ../THIRD_PARTY_NOTICES.md (rustls).

// ANCHOR: listing
// A teaching stand-in, not certificates or trust validation.
#[derive(Debug, PartialEq, Eq)]
pub struct RootStore(String);

impl RootStore {
    pub fn new(label: String) -> Self {
        Self(label)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ClientConfig {
    pub roots: RootStore,
}

#[derive(Debug, Default)]
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
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_roots_is_a_runtime_error() {
        assert_eq!(ConfigBuilder::default().build(), Err("missing roots"));
    }

    #[test]
    fn configured_build_moves_the_supplied_store() {
        let roots = RootStore::new("workshop roots".to_owned());
        let allocation = roots.0.as_ptr();
        let config = ConfigBuilder::default().with_roots(roots).build().unwrap();
        assert_eq!(config.roots.0, "workshop roots");
        assert_eq!(config.roots.0.as_ptr(), allocation);
    }

    #[test]
    fn repeated_settings_keep_the_last_store() {
        let config = ConfigBuilder::default()
            .with_roots(RootStore::new("first".to_owned()))
            .with_roots(RootStore::new("second".to_owned()))
            .build()
            .unwrap();
        assert_eq!(config.roots, RootStore::new("second".to_owned()));
    }

    #[test]
    fn supplying_an_empty_store_is_not_missing_roots() {
        let config = ConfigBuilder::default()
            .with_roots(RootStore::new(String::new()))
            .build()
            .unwrap();
        assert_eq!(config.roots, RootStore::new(String::new()));
    }
}
// ANCHOR_END: tests
