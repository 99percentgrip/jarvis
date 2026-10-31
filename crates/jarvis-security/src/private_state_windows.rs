//! Native owner/DACL checks through safe Windows permission wrappers.
//! Only new or explicitly empty application-private directories may be secured.
use std::{io, path::Path};
use windows_permissions::{
    LocalBox, SecurityDescriptor, Sid,
    constants::{AceType, SeObjectType, SecurityInformation},
    utilities::current_process_sid,
    wrappers::{GetSecurityInfo, SetSecurityInfo},
};

fn denied() -> io::Error {
    io::Error::new(
        io::ErrorKind::PermissionDenied,
        "private directory owner or ACL refused",
    )
}

fn handle(path: &Path, write_acl: bool) -> io::Result<std::fs::File> {
    use std::os::windows::fs::{MetadataExt, OpenOptionsExt};
    let mut options = std::fs::OpenOptions::new();
    // READ_CONTROL, optionally WRITE_DAC; pin object and open a reparse point itself.
    options
        .access_mode(0x0002_0000 | if write_acl { 0x0004_0000 } else { 0 })
        .share_mode(0x0000_0001 | 0x0000_0002)
        .custom_flags(0x0200_0000 | 0x0020_0000);
    let file = options.open(path)?;
    if file.metadata()?.file_attributes() & 0x400 != 0 {
        return Err(denied());
    }
    Ok(file)
}

fn descriptor(file: &std::fs::File) -> io::Result<LocalBox<SecurityDescriptor>> {
    GetSecurityInfo(
        file,
        SeObjectType::SE_FILE_OBJECT,
        SecurityInformation::Owner | SecurityInformation::Dacl,
    )
}

fn trusted(sid: &Sid, user: &Sid) -> io::Result<bool> {
    let system: LocalBox<Sid> = "S-1-5-18".parse()?;
    let administrators: LocalBox<Sid> = "S-1-5-32-544".parse()?;
    Ok(sid == user || sid == &*system || sid == &*administrators)
}

/// Read-only verification: null ACLs and unknown/granting third-party ACEs refuse.
pub(super) fn verify(path: &Path) -> io::Result<()> {
    let user = current_process_sid()?;
    let file = handle(path, false)?;
    let sd = descriptor(&file)?;
    if !trusted(sd.owner().ok_or_else(denied)?, &user)? {
        return Err(denied());
    }
    let acl = sd.dacl().ok_or_else(denied)?;
    for index in 0..acl.len() {
        let ace = acl.get_ace(index).ok_or_else(denied)?;
        if ace.ace_type() == AceType::ACCESS_DENIED_ACE_TYPE {
            continue;
        }
        if ace.ace_type() != AceType::ACCESS_ALLOWED_ACE_TYPE
            || !trusted(ace.sid().ok_or_else(denied)?, &user)?
        {
            return Err(denied());
        }
    }
    Ok(())
}

/// Protect only a fresh/empty application-owned root; never alter its parent ACL.
pub(super) fn secure_empty(path: &Path) -> io::Result<()> {
    let user = current_process_sid()?;
    let mut file = handle(path, true)?;
    let old = descriptor(&file)?;
    if !trusted(old.owner().ok_or_else(denied)?, &user)? {
        return Err(denied());
    }
    let sd: LocalBox<SecurityDescriptor> = format!(
        "D:P(A;OICI;FA;;;{})(A;OICI;FA;;;SY)(A;OICI;FA;;;BA)",
        &*user
    )
    .parse()?;
    SetSecurityInfo(
        &mut file,
        SeObjectType::SE_FILE_OBJECT,
        SecurityInformation::Dacl | SecurityInformation::ProtectedDacl,
        None,
        None,
        sd.dacl(),
        None,
    )?;
    verify(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinned_private_verifier_must_not_block_atomic_replacement() {
        let fixture = tempfile::tempdir().unwrap();
        let root = fixture.path().join("private");
        let target = root.join("settings.json");
        crate::write_private_atomic(&target, b"before").unwrap();
        // Hold the exact production verification handle, not an imitation or a
        // timed reader. Before FILE_SHARE_DELETE, MoveFileExW cannot replace it.
        let verifier = handle(&target, false).unwrap();
        crate::write_private_atomic(&target, b"after").unwrap();
        assert_eq!(std::fs::read(&target).unwrap(), b"after");
        drop(verifier);
        crate::validate_private_file(&target).unwrap();
        crate::validate_owned_directory(&root).unwrap();
        assert_eq!(std::fs::read_dir(&root).unwrap().count(), 2);
    }

    #[test]
    fn native_acl_refuses_broad_access_and_secures_only_new_storage() {
        let fixture = tempfile::tempdir().unwrap();
        let private = fixture.path().join("private");
        std::fs::create_dir(&private).unwrap();
        secure_empty(&private).unwrap();
        verify(&private).unwrap();
        let user = current_process_sid().unwrap();
        let broad: LocalBox<SecurityDescriptor> =
            format!("D:P(A;OICI;FA;;;{})(A;OICI;FA;;;WD)", &*user)
                .parse()
                .unwrap();
        let mut file = handle(&private, true).unwrap();
        SetSecurityInfo(
            &mut file,
            SeObjectType::SE_FILE_OBJECT,
            SecurityInformation::Dacl | SecurityInformation::ProtectedDacl,
            None,
            None,
            broad.dacl(),
            None,
        )
        .unwrap();
        assert!(verify(&private).is_err());
        drop(file);
        secure_empty(&private).unwrap();
        verify(&private).unwrap();
    }

    #[test]
    fn native_junction_cannot_redirect_private_initialization() {
        let fixture = tempfile::tempdir().unwrap();
        let source = fixture.path().join("source");
        let junction = fixture.path().join("redirect");
        std::fs::create_dir(&source).unwrap();
        std::fs::write(source.join("canary"), b"unchanged").unwrap();
        let output = std::process::Command::new("powershell.exe")
            .args(["-NoProfile", "-NonInteractive", "-Command",
                "$ErrorActionPreference='Stop'; New-Item -ItemType Junction -Path $env:FIXTURE_JUNCTION -Target $env:FIXTURE_TARGET | Out-Null"])
            .env("FIXTURE_JUNCTION", &junction)
            .env("FIXTURE_TARGET", &source)
            .output().unwrap();
        assert!(output.status.success(), "junction fixture setup failed");
        assert!(crate::prepare_owned_directory(&junction).is_err());
        assert!(!source.join(".jarvis-owner").exists());
        assert_eq!(std::fs::read(source.join("canary")).unwrap(), b"unchanged");
        std::fs::remove_dir(junction).unwrap();
    }
}
