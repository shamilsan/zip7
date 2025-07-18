use std::path::PathBuf;

use zip7::Zip7Archive;

const FILES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/files");

#[test]
fn shrink() {
    let _archive = Zip7Archive::new(PathBuf::from(FILES_DIR).join("shrink.zip"), None).unwrap();
}
