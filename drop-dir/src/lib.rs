// Copyright 2020 by Matthew James Briggs
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(test)]
extern crate uuid;

use std::fs;
use std::ops::Drop;
use std::path::PathBuf;

/// Represents a Directory that will be automatically deleted when it goes out of scope.
///
/// # Example
///
/// ```
/// use std::path::PathBuf;
/// use drop_dir::DropDir;
/// use std::fs::File;
///
/// let drop_dir = DropDir::new(PathBuf::from("/tmp/some/path")).unwrap();
/// let mut file = File::create(drop_dir.path().join("file.txt")).unwrap();
/// // drop_dir deleted when it goes out of scope.
/// ```
/// ## Limitation
///
/// In the example above, only the last component of the `drop_dir` is removed.
/// That is, the dir `/tmp/some/temp/path` is deleted, but `/tmp/some/temp` remains.
/// Any other behavior would get complicated.
///
pub struct DropDir {
    path_buf: PathBuf,
}

impl Drop for DropDir {
    fn drop(&mut self) {
        let result = fs::remove_dir_all(&self.path_buf);
        if result.is_err() {
            println!(
                "Could not delete directory '{}': {}",
                self.path_buf.to_string_lossy(),
                result.err().unwrap()
            );
        }
    }
}

impl DropDir {
    /// # new
    ///
    /// Constructs a new DropDir object from a PathBuf.
    /// Creates the directory at PathBuf if it does not exist (using `fs::create_dir_all`).
    ///
    /// # Example
    /// ```
    /// # use std::path::PathBuf;
    /// # use drop_dir::DropDir;
    /// # use std::fs::File;
    /// let drop_dir = DropDir::new(PathBuf::from("/tmp/some/path")).unwrap();
    /// ```
    pub fn new(path_buf: PathBuf) -> Result<DropDir, std::io::Error> {
        fs::create_dir_all(&path_buf)?;
        Ok(DropDir { path_buf })
    }

    /// # path
    ///
    /// Returns a clone of the internally held PathBuf.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::path::PathBuf;
    /// # use drop_dir::DropDir;
    /// # use std::fs::File;
    /// # let drop_dir = DropDir::new(PathBuf::from("/tmp/some/path")).unwrap();
    /// let path_str = drop_dir.path().to_string_lossy();
    /// ```
    pub fn path(&self) -> PathBuf {
        self.path_buf.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use uuid::Uuid;

    use super::*;

    #[test]
    fn test() {
        let uuid = Uuid::new_v4().to_string();
        let path_buf = std::env::temp_dir().join(uuid);
        // Create a drop_dir within a scope
        {
            let _drop_dir = DropDir::new(path_buf.clone()).unwrap();
            assert!(Path::new(&path_buf).exists())
        }
        assert!(!Path::new(&path_buf).exists())
    }
}
