use std::fs;
use std::path::PathBuf;

use zip7::Zip7Archive;

const FILES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/files");

#[test]
fn shrink() {
    let mut archive = Zip7Archive::new(PathBuf::from(FILES_DIR).join("shrink.zip"), None).unwrap();
    assert_eq!(archive.len(), 1);

    let tempdir = tempfile::tempdir().unwrap();
    let item = archive.into_iter().next().unwrap();
    assert!(!item.is_directory());
    assert_eq!(item.path(), PathBuf::from("FIRST.TXT"));

    let path = tempdir.path().join("0");
    item.set_out_path(&path).unwrap();
    archive.extract().unwrap();
    assert_eq!(
        fs::read_to_string(path).unwrap(),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/files/shrink.txt"
        ))
    );
}
