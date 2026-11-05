//! Does this path is inside given folder?
//!
//! Solution highlights:
//! - Avoids allocating a temporary string
//! - iterators are lazy evalued, but needs additional check

pub fn is_inside_folder(path: &str, folder: &str) -> bool {
    path.len() >= folder.len()
        && path
            .chars()
            .zip(folder.chars().chain("/".chars()))
            .all(|(a, b)| a == b)
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
