# drop-dir

A very simple crate for creating RAII directories.

## Example

```rust
use std::path::PathBuf;
use drop_dir::DropDir;
use std::fs::File;

let drop_dir = DropDir::new(PathBuf::from("/tmp/some/path")).unwrap();
let mut file = File::create(drop_dir.path().join("file.txt")).unwrap();
// drop_dir deleted when it goes out of scope.
```

## Limitation

In the example above, only the last component of the `drop_dir` is removed.
That is, the dir `/tmp/some/temp/path` is deleted, but `/tmp/some/temp` remains.
Any other behavior would get complicated.
