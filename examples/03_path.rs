//! Does this path point to a Rust source file?

use std::path::Path;

pub fn is_rust_source_file(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => match ext.to_str() {
            Some(s) => s == "rs",
            None => false,
        },
        None => false,
    }
}

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
