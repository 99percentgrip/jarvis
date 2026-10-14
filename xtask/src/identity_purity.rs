//! Conservative source policy for the data-only identity specification.
pub fn validate(source: &str) -> Result<(), String> {
    for (position, _) in source.match_indices("std::") {
        let tail = &source[position..];
        if !tail.starts_with("std::path::") && !tail.starts_with("std::collections::") {
            return Err("identity source uses an unapproved standard-library boundary".into());
        }
    }
    for term in [
        "println!",
        "eprintln!",
        "print!",
        "eprint!",
        "dbg!",
        "include!",
        "extern ",
    ] {
        if source.contains(term) {
            return Err("identity source has an I/O or external-code boundary".into());
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pure_data_and_paths_pass_but_io_clock_aliases_and_output_refuse() {
        assert!(validate("use std::path::PathBuf; use std::collections::BTreeSet;").is_ok());
        for source in [
            "use std::fs;",
            "use std::{fs as files};",
            "std::process::Command",
            "std::env::var",
            "std::net::TcpStream",
            "std::thread::spawn",
            "std::time::SystemTime",
            "println!(value)",
            "include!(path)",
            "extern crate external",
        ] {
            assert!(validate(source).is_err(), "{source}");
        }
    }
}
