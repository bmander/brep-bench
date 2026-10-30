//! Writes two things for the runner into `OUT_DIR`:
//!
//! * `cases.rs`: a module for every case file under `src/cases/<category>/<name_snake>.rs`, and
//!   `CASES`, the table from each case's id (`<category>/<name>`) to its `build` function. Adding
//!   a case is adding its file.
//! * `TRUCK_REV`: the truck revision this runner is built against, from Cargo.lock.
use std::fmt::Write as _;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=src/cases");
    let lock = std::fs::read_to_string("Cargo.lock").unwrap_or_default();
    let rev = lock
        .split("[[package]]")
        .find(|p| p.contains("name = \"truck-modeling\""))
        .and_then(|p| p.lines().find(|l| l.starts_with("source = ")))
        .and_then(|l| l.rsplit('#').next())
        .map(|s| s.trim_end_matches('"').to_string())
        .unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=TRUCK_REV={rev}");

    let root = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/cases");
    let mut categories = sorted(&root, |p| p.is_dir());
    categories.retain(|c| !c.starts_with('.'));
    let (mut modules, mut table) = (String::new(), String::new());
    modules.push_str("/// One module per case file, `src/cases/<category>/<name_snake>.rs`.\n");
    modules.push_str("pub mod cases {\n");
    for category in &categories {
        let dir = root.join(category);
        let names = sorted(&dir, |p| p.extension().is_some_and(|e| e == "rs"));
        let _ = writeln!(modules, "    pub mod r#{category} {{");
        for file in &names {
            let module = file.trim_end_matches(".rs");
            let path = dir.join(file);
            let _ = writeln!(modules, "        #[path = {:?}]", path.display().to_string());
            let _ = writeln!(modules, "        pub mod r#{module};");
            let id = format!("{category}/{}", module.replace('_', "-"));
            let _ = writeln!(table, "    ({id:?}, cases::r#{category}::r#{module}::build),");
        }
        modules.push_str("    }\n");
    }
    modules.push_str("}\n");
    let out = format!(
        "{modules}\n/// Every case this runner implements: its id, and the function that builds \
         it.\npub const CASES: &[(&str, fn() -> crate::Built)] = &[\n{table}];\n"
    );
    let dest = Path::new(&std::env::var("OUT_DIR").unwrap()).join("cases.rs");
    std::fs::write(dest, out).expect("cases.rs is written");
}

/// The names of the entries of `dir` for which `keep` holds, sorted.
fn sorted(dir: &Path, keep: impl Fn(&Path) -> bool) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|e| keep(&e.path()))
                .filter_map(|e| e.file_name().into_string().ok())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}
