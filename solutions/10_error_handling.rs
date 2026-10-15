//! Read the entire contents of a file into a `String`.
//!
//! Solution highlights:
//! - `match … Err(e) => return Err(e)` is literally what `?` expands to.
//! - `std::fs::read_to_string` opens, reads, and closes the file for us.
//! - The function is now small enough that it almost stops needing to exist.

use std::fs;
use std::io;

pub fn read_file_contents(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    fn write_tmp(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!("refactoring_14_{}", name));
        let mut f = File::create(&path).unwrap();
        f.write_all(contents.as_bytes()).unwrap();
        path
    }

    #[test]
    fn reads_existing_file() {
        let path = write_tmp("hello.txt", "hello, world");
        let path = path.to_str().unwrap();
        assert_eq!(read_file_contents(path).unwrap(), "hello, world");
    }

    #[test]
    fn missing_file_is_error() {
        let err = read_file_contents("/definitely/does/not/exist.txt").unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
