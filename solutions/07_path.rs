// ANCHOR: listing
use std::path::Path;

/// Does this path point to a Rust source file?
///
/// Solution highlights:
/// - `is_some_and` collapses both `None` arms into a single `false`.
/// - `OsStr` implements `PartialEq<str>`, so no `to_str` dance is required.
/// - One line, no nesting, same semantics.
pub fn is_rust_source_file(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "rs")
}

// Equally fine:
// pub fn is_rust_source_file(path: &Path) -> bool {
//     path.extension().map_or(false, |e| e == "rs")
// }

// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_rs_file() {
        assert!(is_rust_source_file(Path::new("main.rs")));
    }

    #[test]
    fn nested_rs_file() {
        assert!(is_rust_source_file(Path::new("src/lib.rs")));
    }

    #[test]
    fn other_extension() {
        assert!(!is_rust_source_file(Path::new("Cargo.toml")));
    }

    #[test]
    fn no_extension() {
        assert!(!is_rust_source_file(Path::new("README")));
    }

    #[test]
    fn dotfile_without_extension() {
        assert!(!is_rust_source_file(Path::new(".rs")));
    }
}
// ANCHOR_END: tests
