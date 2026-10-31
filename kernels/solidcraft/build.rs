//! Finds the case implementations and the SolidCraft version for the runner.
//!
//! Every `src/cases/<category>/<name_snake>.rs` is one case, `<category>/<name>` with the name's
//! underscores as hyphens, and must define `pub fn build() -> Result<Body, E>`, where `E` is the
//! kernel's `KernelError` or, for a case that refuses in its own words, the runner's `Refusal`.
//! This writes `$OUT_DIR/cases.rs`: a module per file (a raw identifier, so `box.rs` is one) and
//! `CASES`, the table from case id to its `build`, its error made a `Refusal`. Adding a case is
//! adding its file.
//!
//! It also sets `SOLIDCRAFT_VERSION` to the version and commit of `solidcraft-kernel` in
//! `Cargo.lock`.
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cases = root.join("src/cases");
    println!("cargo:rerun-if-changed={}", cases.display());

    let mut categories: Vec<_> = fs::read_dir(&cases)
        .expect("src/cases reads")
        .map(|e| e.expect("a directory entry").path())
        .filter(|p| p.is_dir())
        .collect();
    categories.sort();

    let (mut modules, mut table) = (String::new(), String::new());
    for dir in &categories {
        println!("cargo:rerun-if-changed={}", dir.display());
        let category = dir.file_name().unwrap().to_str().expect("a UTF-8 category");
        let mut files: Vec<_> = fs::read_dir(dir)
            .expect("a category reads")
            .map(|e| e.expect("a directory entry").path())
            .filter(|p| p.extension().is_some_and(|x| x == "rs"))
            .collect();
        files.sort();
        writeln!(modules, "pub mod {category} {{").unwrap();
        for file in &files {
            let name = file.file_stem().unwrap().to_str().expect("a UTF-8 name");
            writeln!(modules, "    #[path = {:?}]\n    pub mod r#{name};", file.display().to_string())
                .unwrap();
            writeln!(
                table,
                "    ({:?}, || {category}::r#{name}::build().map_err(Refusal::from)),",
                format!("{category}/{}", name.replace('_', "-"))
            )
            .unwrap();
        }
        writeln!(modules, "}}").unwrap();
    }
    let out = Path::new(&std::env::var("OUT_DIR").unwrap()).join("cases.rs");
    let text = format!(
        "{modules}\n/// Every case id and the function that builds it.\npub const CASES: &[(&str, \
         Build)] = &[\n{table}];\n"
    );
    fs::write(out, text).expect("cases.rs writes");

    println!("cargo:rerun-if-changed=Cargo.lock");
    let lock = fs::read_to_string(root.join("Cargo.lock")).unwrap_or_default();
    let package = lock.split("[[package]]").find(|p| p.contains("name = \"solidcraft-kernel\"\n"));
    let field = |key: &str| {
        package
            .and_then(|p| p.lines().find_map(|l| l.strip_prefix(key)))
            .map(|v| v.trim_end_matches('"').to_owned())
    };
    let version = field("version = \"").unwrap_or_else(|| "unknown".into());
    let commit = field("source = \"")
        .and_then(|s| s.rsplit_once('#').map(|(_, c)| c.chars().take(12).collect::<String>()));
    let version = match commit {
        Some(c) => format!("{version} ({c})"),
        None => version,
    };
    println!("cargo:rustc-env=SOLIDCRAFT_VERSION={version}");
}
