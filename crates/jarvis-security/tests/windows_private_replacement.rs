#![cfg(windows)]

use jarvis_security::{validate_owned_directory, validate_private_file, write_private_atomic};
use std::{fs, os::windows::fs::OpenOptionsExt};

#[test]
fn non_delete_sharing_reader_denies_replace_without_corrupting_the_record() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path().join("private");
    let target = root.join("settings.json");
    let original = vec![b'a'; 8192];
    let replacement = vec![b'b'; 8192];
    write_private_atomic(&target, &original).unwrap();

    // Share read/write but intentionally NOT delete: Windows rename-over-target
    // must fail while this handle is held. The barrier is the live handle,
    // not timing or a retry loop. Neither failure nor cleanup may truncate it.
    let reader = fs::OpenOptions::new()
        .read(true)
        .share_mode(0x1 | 0x2)
        .open(&target)
        .unwrap();
    let error = write_private_atomic(&target, &replacement).unwrap_err();
    assert!(
        matches!(error.raw_os_error(), Some(5 | 32)),
        "unexpected replace failure: {error}"
    );
    assert_eq!(fs::read(&target).unwrap(), original);
    validate_owned_directory(&root).unwrap();
    drop(reader);

    // A new operation, after the conflicting handle closes, must publish a
    // complete record; this is not an automatic retry of the failed write.
    write_private_atomic(&target, &replacement).unwrap();
    assert_eq!(fs::read(&target).unwrap(), replacement);
    validate_private_file(&target).unwrap();
    validate_owned_directory(&root).unwrap();
    assert_eq!(fs::read_dir(&root).unwrap().count(), 2);
}
