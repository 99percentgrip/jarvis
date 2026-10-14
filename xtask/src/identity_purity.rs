//! Conservative source policy for the data-only identity specification.
pub fn validate(source: &str) -> Result<(), String> {
    let compact = source.split_whitespace().collect::<String>();
    if compact.contains("usestdas") || compact.contains("use::stdas") {
        return Err("identity source aliases the standard-library root".into());
    }
    for (position, _) in compact.match_indices("std::") {
        let tail = &compact[position..];
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
        "externcrate",
        "extern\"",
        "include_str!",
        "include_bytes!",
        "env!",
        "option_env!",
        ".exists(",
        ".try_exists(",
        ".is_file(",
        ".is_dir(",
        ".metadata(",
        ".symlink_metadata(",
        ".read_dir(",
        ".canonicalize(",
        ".read_link(",
    ] {
        if compact.contains(term) {
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
            "use std :: fs;",
            "path . metadata ()",
            "use std as runtime;",
            "use ::std as runtime;",
            "std::path::Path::new(path).exists()",
            "path.canonicalize()",
            "env!(key)",
            "include_str!(path)",
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
