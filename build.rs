use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::process;

fn main() {
    // OUT_DIR is set by Cargo and it's where any additional build artifacts
    // are written.
    let outdir = match env::var_os("OUT_DIR") {
        Some(outdir) => outdir,
        None => {
            eprintln!(
                "OUT_DIR environment variable not defined. \
                 This should have been set by Cargo.");
            process::exit(1);
        }
    };
    fs::create_dir_all(&outdir).unwrap();
    let stamp_path = Path::new(&outdir).join("stamp");
    if let Err(err) = File::create(&stamp_path) {
        panic!("failed to write {}: {}", stamp_path.display(), err);
    }
    // Make the current git hash available to the build.
    if let Some(rev) = git_revision_hash() {
        println!("cargo:rustc-env=CARGO_BUILD_GIT_HASH={}", rev);
    }
}

fn git_revision_hash() -> Option<String> {
    let result = process::Command::new("git")
        .args(&["rev-parse", "--short=10", "HEAD"])
        .output();
    result.ok().and_then(|output| {
        let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if v.is_empty() {
            None
        } else {
            Some(v)
        }
    })
}