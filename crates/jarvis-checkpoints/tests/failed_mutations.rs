//! Disposable production-store regressions: a failed write must not publish cached state.
use jarvis_checkpoints::{CronRegistry, SessionLineage, SessionStatus};
use std::fs;
use tempfile::TempDir;

#[test]
fn failed_cron_rewrites_leave_live_cache_and_reopened_store_unchanged() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("owned");
    fs::create_dir(&root).unwrap();
    let cron = CronRegistry::open(&root).unwrap();
    let job = cron.register("before", "prompt", "daily").unwrap();
    let log = root.join("cron.jsonl");
    let backup = root.join("cron.saved");
    fs::rename(&log, &backup).unwrap();
    fs::create_dir(&log).unwrap(); // Force atomic rename to fail, on every OS.
    assert!(cron.update(&job.id, Some("after"), None, None).is_err());
    assert!(cron.set_enabled(&job.id, false).is_err());
    assert!(cron.forget(&job.id).is_err());
    assert!(cron.claim_due(std::time::SystemTime::now(), true).is_err());
    assert!(cron.register("retry", "prompt", "daily").is_err());
    assert_eq!(cron.get(&job.id).unwrap().name, "before");
    assert!(cron.get(&job.id).unwrap().enabled);
    fs::remove_dir(&log).unwrap();
    fs::rename(&backup, &log).unwrap();
    let reopened = CronRegistry::open(&root).unwrap();
    assert_eq!(reopened.get(&job.id).unwrap().name, "before");
    assert!(reopened.get(&job.id).unwrap().enabled);
    assert_eq!(
        cron.register("retry", "prompt", "daily").unwrap().id,
        "cron-2"
    );
    assert_eq!(
        cron.claim_due(std::time::SystemTime::now(), true)
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn failed_cron_lease_updates_preserve_the_existing_claim() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("owned");
    fs::create_dir(&root).unwrap();
    let cron = CronRegistry::open(&root).unwrap();
    let job = cron.register("recurring", "prompt", "daily").unwrap();
    let now = std::time::SystemTime::now();
    let claimed = cron.claim_due(now, true).unwrap().remove(0);
    let original = cron.get(&job.id).unwrap();
    let log = root.join("cron.jsonl");
    let backup = root.join("cron.saved");
    fs::rename(&log, &backup).unwrap();
    fs::create_dir(&log).unwrap();
    assert!(cron.renew_claim(&job.id, &claimed.token, now).is_err());
    assert!(
        cron.finish_claim(&job.id, &claimed.token, "ok", "done")
            .is_err()
    );
    assert_eq!(cron.get(&job.id).unwrap().claim, original.claim);
    assert_eq!(cron.get(&job.id).unwrap().run_count, 0);
    fs::remove_dir(&log).unwrap();
    fs::rename(&backup, &log).unwrap();
    assert_eq!(
        CronRegistry::open(&root)
            .unwrap()
            .get(&job.id)
            .unwrap()
            .claim,
        original.claim
    );
    assert_eq!(
        cron.finish_claim(&job.id, &claimed.token, "ok", "done")
            .unwrap()
            .run_count,
        1
    );
}

#[test]
fn failed_session_rewrite_does_not_publish_or_supersede_in_memory() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("owned");
    fs::create_dir(&root).unwrap();
    let sessions = SessionLineage::open(&root).unwrap();
    let parent = sessions.create(None, Some("before"), temp.path()).unwrap();
    let log = root.join("sessions.jsonl");
    let backup = root.join("sessions.saved");
    fs::rename(&log, &backup).unwrap();
    fs::create_dir(&log).unwrap();
    assert!(sessions.rename(&parent.id, "after").is_err());
    assert!(
        sessions
            .create(Some(&parent.id), Some("child"), temp.path())
            .is_err()
    );
    assert!(
        sessions
            .branch(&parent.id, Some("branch"), temp.path())
            .is_err()
    );
    assert_eq!(sessions.get(&parent.id).unwrap().name, "before");
    assert_eq!(
        sessions.get(&parent.id).unwrap().status,
        SessionStatus::Active
    );
    assert_eq!(sessions.len(), 1);
    fs::remove_dir(&log).unwrap();
    fs::rename(&backup, &log).unwrap();
    let reopened = SessionLineage::open(&root).unwrap();
    assert_eq!(reopened.get(&parent.id).unwrap().name, "before");
    assert_eq!(reopened.len(), 1);
    assert_eq!(
        sessions
            .branch(&parent.id, Some("branch"), temp.path())
            .unwrap()
            .id,
        "sess-2"
    );
}
