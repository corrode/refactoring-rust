// ANCHOR: listing
//! Own a scratch file and attempt cleanup when its owner is dropped.
//!
//! Use a private, stable parent directory controlled by the caller.
//! This is not a secure temp-name allocator: cleanup follows a path and cannot
//! track external renames or replacements. An absolute path avoids dependence
//! on later working-directory changes, but does not protect file identity.
//! Cleanup is best effort and never replaces the operation's result.

use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

#[must_use = "keep the temporary file alive while its path is in use"]
#[derive(Debug)]
pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    pub fn create(path: &Path) -> io::Result<Self> {
        let path = std::path::absolute(path)?;
        File::create_new(&path)?;
        Ok(Self { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

pub fn read_back(path: &Path, bytes: &[u8]) -> io::Result<String> {
    let scratch = TempFile::create(path)?;
    fs::write(scratch.path(), bytes)?;
    fs::read_to_string(scratch.path())
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
    fn scope_exit_removes_scratch_file() {
        let temp = TempDir::new();
        let path = temp.0.join("scratch.txt");
        {
            let scratch = TempFile::create(&path).unwrap();
            assert!(scratch.path().is_absolute());
            fs::write(scratch.path(), b"scratch").unwrap();
            assert_eq!(fs::read_to_string(scratch.path()).unwrap(), "scratch");
        }
        assert!(!path.exists());
    }

    #[test]
    fn creation_collision_preserves_existing_file() {
        let temp = TempDir::new();
        let path = temp.0.join("existing.txt");
        fs::write(&path, b"keep this file").unwrap();
        assert_eq!(
            TempFile::create(&path).unwrap_err().kind(),
            io::ErrorKind::AlreadyExists,
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), "keep this file");
    }

    #[test]
    fn missing_parent_is_an_error() {
        let temp = TempDir::new();
        let path = temp.0.join("missing").join("scratch.txt");
        assert_eq!(
            TempFile::create(&path).unwrap_err().kind(),
            io::ErrorKind::NotFound,
        );
        assert!(!path.exists());
    }

    #[test]
    fn read_error_removes_scratch_file() {
        let temp = TempDir::new();
        let path = temp.0.join("scratch.txt");
        let error = read_back(&path, &[0xff]).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert!(
            !path.exists(),
            "scratch file must be removed when reading fails",
        );
    }

    #[test]
    fn moving_guard_keeps_file_until_new_owner_is_dropped() {
        let temp = TempDir::new();
        let path = temp.0.join("scratch.txt");
        let scratch = {
            let scratch = TempFile::create(&path).unwrap();
            fs::write(scratch.path(), b"still here").unwrap();
            scratch
        };
        assert_eq!(fs::read_to_string(&path).unwrap(), "still here");
        drop(scratch);
        assert!(!path.exists());
    }

    #[test]
    fn question_mark_removes_scratch_file() {
        fn read_missing_file(path: &Path) -> io::Result<()> {
            let scratch = TempFile::create(path)?;
            fs::read(scratch.path().with_extension("missing"))?;
            Ok(())
        }

        let temp = TempDir::new();
        let path = temp.0.join("scratch.txt");
        let error = read_missing_file(&path).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::NotFound);
        assert!(!path.exists());
    }

    #[test]
    fn early_return_removes_scratch_file() {
        fn write_if_enabled(path: &Path, enabled: bool) -> io::Result<()> {
            let scratch = TempFile::create(path)?;
            if !enabled {
                return Ok(());
            }
            fs::write(scratch.path(), b"finished")
        }

        let temp = TempDir::new();
        let path = temp.0.join("scratch.txt");
        write_if_enabled(&path, false).unwrap();
        assert!(!path.exists());
    }

    #[cfg(panic = "unwind")]
    #[test]
    fn unwinding_removes_scratch_file() {
        let temp = TempDir::new();
        let path = temp.0.join("scratch.txt");
        let scratch = TempFile::create(&path).unwrap();
        let result = std::panic::catch_unwind(move || {
            let _scratch = scratch;
            panic!("stop while using the scratch file");
        });
        assert!(result.is_err());
        assert!(!path.exists());
    }

    #[test]
    fn removing_missing_file_does_not_panic() {
        let temp = TempDir::new();
        let path = temp.0.join("scratch.txt");
        let scratch = TempFile::create(&path).unwrap();
        fs::remove_file(scratch.path()).unwrap();
        drop(scratch);
        assert!(!path.exists());
    }

    #[test]
    fn copying_path_does_not_keep_file_alive() {
        let temp = TempDir::new();
        let path = temp.0.join("scratch.txt");
        let copied_path = {
            let scratch = TempFile::create(&path).unwrap();
            scratch.path().to_path_buf()
        };
        assert_eq!(copied_path, std::path::absolute(&path).unwrap());
        assert!(!copied_path.exists());
    }
}
// ANCHOR_END: tests
