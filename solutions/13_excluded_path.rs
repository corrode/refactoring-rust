//! Does `path` start with any of the excluded prefixes?
//!
//! Solution highlights:
//! - `let ... else` handles the "no exclusions" case up front, no nesting.
//! - `iter().any(...)` is the idiomatic spelling of "does any element match?"
//! - Even better, if the API allows it: take `&HashSet<PathBuf>` and let the
//!   empty set carry the "nothing excluded" meaning.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn is_excluded(excluded: &Option<HashSet<PathBuf>>, path: &Path) -> bool {
    let Some(excluded) = excluded else {
        return false;
    };
    excluded.iter().any(|prefix| path.starts_with(prefix))
}

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
