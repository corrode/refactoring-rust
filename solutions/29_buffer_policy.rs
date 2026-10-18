//! Workshop redesign of buffer configuration adapted from hyper.[^source]
//! This enum is not a claimed upstream refactor or bug fix.
//! Copyright (c) 2014-2021 Sean McArthur. MIT notice: ../THIRD_PARTY_NOTICES.md.
//!
//! [^source]: https://github.com/hyperium/hyper/blob/73bff4e98c372ce04b006370c0b0d2af29ea8718/src/client/conn.rs#L616-L642

// ANCHOR: listing
#[derive(Debug, Default, PartialEq)]
pub enum BufferConfig {
    #[default]
    Adaptive,
    Exact(usize),
    AdaptiveWithMax(usize),
}

impl BufferConfig {
    pub fn exact_size(&mut self, size: Option<usize>) {
        *self = match size {
            Some(size) => Self::Exact(size),
            None => Self::Adaptive,
        };
    }

    pub fn max_size(&mut self, max: usize) {
        assert!(max >= 8192, "maximum buffer size must be at least 8192");
        *self = Self::AdaptiveWithMax(max);
    }
}
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{AssertUnwindSafe, catch_unwind};

    fn policy(config: &BufferConfig) -> (Option<usize>, Option<usize>) {
        match config {
            BufferConfig::Adaptive => (None, None),
            BufferConfig::Exact(size) => (Some(*size), None),
            BufferConfig::AdaptiveWithMax(max) => (None, Some(*max)),
        }
    }

    #[test]
    fn default_is_adaptive() {
        assert_eq!(policy(&BufferConfig::default()), (None, None));
    }

    #[test]
    fn exact_sizes_are_stored_without_validation() {
        let mut config = BufferConfig::default();
        for size in [1, 4096, 8192, usize::MAX] {
            config.exact_size(Some(size));
            assert_eq!(policy(&config), (Some(size), None));
        }
    }

    #[test]
    fn maxima_are_stored() {
        let mut config = BufferConfig::default();
        for max in [8193, 16384, usize::MAX] {
            config.max_size(max);
            assert_eq!(policy(&config), (None, Some(max)));
        }
    }

    #[test]
    fn exact_then_max_uses_max() {
        let mut config = BufferConfig::default();
        config.exact_size(Some(4096));
        config.max_size(16384);
        assert_eq!(policy(&config), (None, Some(16384)));
    }

    #[test]
    fn max_then_exact_uses_exact() {
        let mut config = BufferConfig::default();
        config.max_size(16384);
        config.exact_size(Some(4096));
        assert_eq!(policy(&config), (Some(4096), None));
    }

    #[test]
    fn exact_none_resets_every_policy() {
        for (exact, max) in [(None, None), (Some(4096), None), (None, Some(16384))] {
            let mut config = BufferConfig::default();
            config.exact_size(exact);
            if let Some(max) = max {
                config.max_size(max);
            }
            config.exact_size(None);
            assert_eq!(policy(&config), (None, None));
        }
    }

    #[test]
    fn zero_exact_size_is_accepted() {
        let mut config = BufferConfig::default();
        config.max_size(16384);
        config.exact_size(Some(0));
        assert_eq!(policy(&config), (Some(0), None));
    }

    #[test]
    fn minimum_maximum_is_accepted() {
        let mut config = BufferConfig::default();
        config.exact_size(Some(4096));
        config.max_size(8192);
        assert_eq!(policy(&config), (None, Some(8192)));
    }

    #[test]
    fn invalid_maximum_panics_without_changing_state() {
        for invalid in [0, 1, 8191] {
            for (exact, max) in [(None, None), (Some(4096), None), (None, Some(16384))] {
                let mut config = BufferConfig::default();
                config.exact_size(exact);
                if let Some(max) = max {
                    config.max_size(max);
                }
                let before = policy(&config);
                let result = catch_unwind(AssertUnwindSafe(|| config.max_size(invalid)));
                assert!(result.is_err());
                assert_eq!(policy(&config), before);
            }
        }
    }
}
// ANCHOR_END: tests

fn main() {}
