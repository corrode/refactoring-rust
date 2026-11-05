//! Does this path is inside given folder?
//!
//! Solution highlights:
//! - Avoids allocating a temporary string
//! - `is_some_and` collapses both `None` arms into a single `false`.
//! - One line, no nesting, same semantics.

pub fn is_inside_folder(path: &str, folder: &str) -> bool {
    path.strip_prefix(folder)
        .is_some_and(|sub| sub.starts_with("/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert!(is_inside_folder("/path/to/file.rs", "/path/to"));
    }

    #[test]
    fn different_folder() {
        assert!(!is_inside_folder("/path/to/file.rs", "/another/folder"));
    }

    #[test]
    fn similar_folder() {
        assert!(!is_inside_folder("/path/too/file.rs", "/path/to"));
    }

    #[test]
    fn similar_subfolder() {
        assert!(!is_inside_folder("/another/path/to/file.rs", "/path/to"));
    }

    #[test]
    fn parent_folder() {
        assert!(!is_inside_folder("/path", "/path/to"));
    }
}
