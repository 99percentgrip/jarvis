#![cfg(windows)]

use jarvis_security::{validate_owned_directory, validate_private_file, write_private_atomic};
use std::{
    fs,
    os::windows::fs::OpenOptionsExt,
    sync::{Arc, Barrier},
};

fn no_delete_share_reader(target: &std::path::Path) -> fs::File {
    fs::OpenOptions::new()
        .read(true)
        .share_mode(0x1 | 0x2)
        .open(target)
        .unwrap()
}

fn assert_sharing_denied(error: std::io::Error) {
    assert!(
        matches!(error.raw_os_error(), Some(5 | 32)),
        "unexpected replace failure: {error}"
    );
}

#[test]
fn delete_sharing_reader_allows_replacement_without_losing_its_snapshot() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path().join("private");
    let target = root.join("settings.json");
    write_private_atomic(&target, b"original").unwrap();
    for value in 0..32u8 {
        let mut reader = fs::OpenOptions::new()
            .read(true)
            .share_mode(0x1 | 0x2 | 0x4)
            .open(&target)
            .unwrap();
        let before = fs::read(&target).unwrap();
        let next = vec![value; 8192];
        write_private_atomic(&target, &next).unwrap();
        use std::io::Read;
        let mut snapshot = Vec::new();
        reader.read_to_end(&mut snapshot).unwrap();
        assert_eq!(snapshot, before);
        assert_eq!(fs::read(&target).unwrap(), next);
        validate_private_file(&target).unwrap();
        validate_owned_directory(&root).unwrap();
        assert_eq!(fs::read_dir(&root).unwrap().count(), 2);
    }
}

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
    let reader = no_delete_share_reader(&target);
    assert_sharing_denied(write_private_atomic(&target, &replacement).unwrap_err());
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

#[test]
fn synchronized_competing_writers_preserve_committed_record_and_own_staging() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path().join("private");
    let target = root.join("settings.json");
    let original = vec![b'a'; 8192];
    let first = vec![b'b'; 8192];
    let second = vec![b'c'; 8192];
    write_private_atomic(&target, &original).unwrap();

    // Both writers start after the incompatible handle has opened. This is a
    // controlled sharing-denial hypothesis, not reproduction of Issue #23's
    // unknown failing syscall. Neither writer owns the other's staged file.
    let reader = no_delete_share_reader(&target);
    let barrier = Arc::new(Barrier::new(3));
    std::thread::scope(|scope| {
        let attempts: Vec<_> = [&first, &second]
            .into_iter()
            .map(|bytes| {
                let barrier = Arc::clone(&barrier);
                let target = &target;
                scope.spawn(move || {
                    barrier.wait();
                    write_private_atomic(target, bytes)
                })
            })
            .collect();
        barrier.wait();
        for attempt in attempts {
            assert_sharing_denied(attempt.join().unwrap().unwrap_err());
        }
    });
    assert_eq!(fs::read(&target).unwrap(), original);
    validate_owned_directory(&root).unwrap();
    validate_private_file(&target).unwrap();
    // Only the published record and its owner marker survive both failures.
    assert_eq!(fs::read_dir(&root).unwrap().count(), 2);
    drop(reader);

    // Fresh operations succeed after the barrier is gone, without changing
    // the failed attempts' results or hiding a failed write behind a retry.
    for bytes in [&first, &second] {
        write_private_atomic(&target, bytes).unwrap();
        assert_eq!(fs::read(&target).unwrap(), *bytes);
        validate_private_file(&target).unwrap();
        validate_owned_directory(&root).unwrap();
        assert_eq!(fs::read_dir(&root).unwrap().count(), 2);
    }
}
