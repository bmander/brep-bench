//! Sets `POPKERN_VERSION` to the commit of popkern in `Cargo.lock`, which the runner's `--version`
//! names.
use std::fs;
use std::path::Path;

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    println!("cargo:rerun-if-changed=Cargo.lock");
    let lock = fs::read_to_string(root.join("Cargo.lock")).unwrap_or_default();
    let version = lock
        .split("[[package]]")
        .find(|p| p.contains("name = \"popkern-rhai\"\n"))
        .and_then(|p| p.lines().find_map(|l| l.strip_prefix("source = \"")))
        .and_then(|s| s.trim_end_matches('"').rsplit_once('#'))
        .map_or("unknown".to_owned(), |(_, commit)| commit[..commit.len().min(12)].to_owned());
    println!("cargo:rustc-env=POPKERN_VERSION={version}");
}
