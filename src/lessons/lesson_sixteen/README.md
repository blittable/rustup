# Lesson Sixteen: Closure Patterns

## Objective: Look at a few interesting uses of closures



```rust
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

fn main() {
    let proto_root = "proto";
    build_protobufs(&proto_root);
}

fn build_protobufs(proto_root: &'static str) {
    let proto_buf_tests_root = Path::new(proto_root);
    visit_dirs(&proto_buf_tests_root, &process_entries::<&DirEntry>).unwrap();
}

//This is our callback, invoked when a file is found.
fn process_entries<F>(f: &DirEntry) {
    println!("Entry: {:?}", f);
    tonic_build::compile_protos(f.path()).unwrap();
}

// recursively get files
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
```
