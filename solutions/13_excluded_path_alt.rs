//! Alternative APIs for path exclusion rules.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn is_excluded(path: &Path, excluded: &HashSet<PathBuf>) -> bool {
    excluded.iter().any(|prefix| path.starts_with(prefix))
}

#[derive(Debug, Default)]
pub struct Excluder {
    prefixes: HashSet<PathBuf>,
}

impl Excluder {
    pub fn new(prefixes: impl IntoIterator<Item = PathBuf>) -> Self {
        Self {
            prefixes: prefixes.into_iter().collect(),
        }
    }

    pub fn is_excluded(&self, path: &Path) -> bool {
        self.prefixes.iter().any(|prefix| path.starts_with(prefix))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(paths: &[&str]) -> HashSet<PathBuf> {
        paths.iter().map(PathBuf::from).collect()
    }

    #[test]
    fn plain_function_excludes_paths_below_a_prefix() {
        let excluded = set(&["/tmp"]);
        assert!(is_excluded(Path::new("/tmp/foo.log"), &excluded));
        assert!(!is_excluded(Path::new("/home/me/x"), &excluded));
    }

    #[test]
    fn path_prefixes_respect_component_boundaries() {
        let excluded = set(&["/tmp"]);
        assert!(!is_excluded(Path::new("/tmpfile"), &excluded));
    }

    #[test]
    fn an_empty_set_excludes_nothing() {
        assert!(!is_excluded(Path::new("/tmp/foo.log"), &HashSet::new()));
    }

    #[test]
    fn excluder_owns_the_matching_policy() {
        let excluder = Excluder::new([PathBuf::from("/tmp")]);
        assert!(excluder.is_excluded(Path::new("/tmp/foo.log")));
        assert!(!excluder.is_excluded(Path::new("/home/me/x")));
    }

    #[test]
    fn default_excluder_excludes_nothing() {
        assert!(!Excluder::default().is_excluded(Path::new("/tmp/foo.log")));
    }
}
