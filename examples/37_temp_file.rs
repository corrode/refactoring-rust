// ANCHOR: listing
//! Create, use, and remove a scratch file, including when an operation fails.
//!
//! Use a private, stable parent directory controlled by the caller.
//! This is not a secure temp-name allocator: cleanup follows a path and cannot
//! track external renames or replacements. An absolute path avoids dependence
//! on later working-directory changes, but does not protect file identity.
//! Cleanup is best effort. You may change the creation API, but keep read_back's
//! signature. The scratch_file_is_removed_after_read_error test intentionally fails.

use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

pub fn create_temp_file(path: &Path) -> io::Result<PathBuf> {
    let path = std::path::absolute(path)?;
    File::create_new(&path)?;
    Ok(path)
}

pub fn read_back(path: &Path, bytes: &[u8]) -> io::Result<String> {
    let path = create_temp_file(path)?;
    fs::write(&path, bytes)?;
    let text = fs::read_to_string(&path)?;
    let _ = fs::remove_file(&path);
    Ok(text)
}
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            for _ in 0..100 {
                let path = std::env::temp_dir().join(format!(
                    "refactoring-temp-file-{}-{timestamp}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed),
                ));
                match fs::create_dir(&path) {
                    Ok(()) => return Self(path),
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                    Err(error) => panic!("create temporary directory {}: {error}", path.display()),
                }
            }
            panic!("could not allocate a unique temporary directory");
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            if let Err(error) = fs::remove_dir_all(&self.0) {
                if error.kind() != io::ErrorKind::NotFound {
                    // Do not cause a second panic while unwinding a failed assertion.
                    eprintln!("remove temporary directory {}: {error}", self.0.display());
                }
            }
        }
    }

    #[test]
    fn successful_round_trip_removes_scratch_file() {
        let temp = TempDir::new();
        let path = temp.0.join("scratch.txt");
        let text = "hello, 世界\n";
        assert_eq!(read_back(&path, text.as_bytes()).unwrap(), text);
        assert!(!path.exists());
    }

    #[test]
    fn caller_can_create_use_and_remove_scratch_file() {
        let temp = TempDir::new();
        let path = create_temp_file(&temp.0.join("scratch.txt")).unwrap();
        assert!(path.is_absolute());
        fs::write(&path, b"scratch").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "scratch");
        fs::remove_file(&path).unwrap();
        assert!(!path.exists());
    }

    #[test]
    fn creation_collision_preserves_existing_file() {
        let temp = TempDir::new();
        let path = temp.0.join("existing.txt");
        fs::write(&path, b"keep this file").unwrap();
        assert_eq!(
            create_temp_file(&path).unwrap_err().kind(),
            io::ErrorKind::AlreadyExists,
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), "keep this file");
    }

    #[test]
    fn missing_parent_is_an_error() {
        let temp = TempDir::new();
        let path = temp.0.join("missing").join("scratch.txt");
        assert_eq!(
            create_temp_file(&path).unwrap_err().kind(),
            io::ErrorKind::NotFound,
        );
        assert!(!path.exists());
    }

    #[test]
    fn scratch_file_is_removed_after_read_error() {
        let temp = TempDir::new();
        let path = temp.0.join("scratch.txt");
        let error = read_back(&path, &[0xff]).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert!(
            !path.exists(),
            "scratch file must be removed when reading fails",
        );
    }
}
// ANCHOR_END: tests
