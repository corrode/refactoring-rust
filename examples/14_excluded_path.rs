// ANCHOR: listing
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Does `path` start with any of the excluded prefixes?
/// `None` means no exclusions are configured (so nothing is excluded).
pub fn is_excluded(excluded: &Option<HashSet<PathBuf>>, path: &Path) -> bool {
    if let Some(excluded) = excluded {
        for prefix in excluded {
            if path.starts_with(prefix) {
                return true;
            }
        }
    }
    false
}

// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    fn set(paths: &[&str]) -> Option<HashSet<PathBuf>> {
        Some(paths.iter().map(PathBuf::from).collect())
    }

    #[test]
    fn excludes_under_prefix() {
        assert!(is_excluded(&set(&["/tmp"]), Path::new("/tmp/foo.log")));
    }

    #[test]
    fn not_under_any_prefix() {
        assert!(!is_excluded(&set(&["/tmp"]), Path::new("/home/me/x")));
    }

    #[test]
    fn none_means_nothing_excluded() {
        assert!(!is_excluded(&None, Path::new("/anything")));
    }

    #[test]
    fn empty_set_excludes_nothing() {
        let ex = Some(HashSet::new());
        assert!(!is_excluded(&ex, Path::new("/tmp/foo")));
    }
}
// ANCHOR_END: tests
