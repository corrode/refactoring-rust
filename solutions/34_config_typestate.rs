// Provenance: instructor-created concrete states, not an upstream excerpt.
// Simplifies rustls's generic typed transition; no refactoring PR is claimed.
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
pub struct NeedsRoots;

#[derive(Debug)]
pub struct Ready {
    roots: RootStore,
}

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
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configured_build_moves_the_supplied_store() {
        let roots = RootStore::new("workshop roots".to_owned());
        let allocation = roots.0.as_ptr();
        let ready = NeedsRoots::default().with_roots(roots);
        let config: ClientConfig = ready.build();
        assert_eq!(config.roots.0, "workshop roots");
        assert_eq!(config.roots.0.as_ptr(), allocation);
    }

    #[test]
    fn supplying_an_empty_store_does_not_validate_contents() {
        let config = NeedsRoots::default()
            .with_roots(RootStore::new(String::new()))
            .build();
        assert_eq!(config.roots, RootStore::new(String::new()));
    }
}
// ANCHOR_END: tests
