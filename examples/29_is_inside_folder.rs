//! Does this path is inside given folder?

pub fn is_inside_folder(path: &str, folder: &str) -> bool {
    path.starts_with(&format!("{folder}/"))
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
