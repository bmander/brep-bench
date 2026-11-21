//! Finds every case file under `src/cases/<category>/<name_snake>.rs` and writes the table the
//! runner looks a case id up in (`$OUT_DIR/cases.rs`): adding a case is adding its file. Also reads
//! which version and commit of anvil-brep `Cargo.lock` resolved, for `--version`.
use std::fmt::Write as _;
use std::path::Path;

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cases = root.join("src/cases");
    println!("cargo:rerun-if-changed=src/cases");
    println!("cargo:rerun-if-changed=Cargo.lock");

    let mut categories: Vec<_> = std::fs::read_dir(&cases)
        .expect("src/cases reads")
        .map(|e| e.expect("a directory entry").path())
        .filter(|p| p.is_dir())
        .collect();
    categories.sort();
    let (mut modules, mut table) = (String::new(), String::new());
    for dir in categories {
        let category = dir.file_name().unwrap().to_str().unwrap().to_string();
        let mut files: Vec<_> = std::fs::read_dir(&dir)
            .expect("a category reads")
            .map(|e| e.expect("a directory entry").path())
            .filter(|p| p.extension().is_some_and(|x| x == "rs"))
            .collect();
        files.sort();
        writeln!(modules, "pub mod r#{category} {{").unwrap();
        for file in files {
            let name = file.file_stem().unwrap().to_str().unwrap().to_string();
            writeln!(
                modules,
                "    #[path = {:?}]\n    pub mod r#{name};",
                file.display().to_string()
            )
            .unwrap();
            writeln!(
                table,
                "    (\"{category}/{}\", || crate::Bodies::bodies(r#{category}::r#{name}::build())),",
                name.replace('_', "-")
            )
            .unwrap();
        }
        writeln!(modules, "}}").unwrap();
    }
    let out = format!(
        "{modules}\n/// Every case by its id, and the function that builds it.\n\
        pub const CASES: &[(&str, fn() -> Result<Vec<anvil_brep::Solid>, String>)] = &[\n{table}];\n"
    );
    let out_dir = std::env::var("OUT_DIR").unwrap();
    std::fs::write(Path::new(&out_dir).join("cases.rs"), out).unwrap();

    // the version the lock file resolved anvil-brep to, `version = "0.1.2"`, and from git the
    // commit, `source = "git+…?rev=…#<sha>"`
    let lock = std::fs::read_to_string(root.join("Cargo.lock")).unwrap_or_default();
    let entry = lock
        .split("[[package]]")
        .find(|p| p.contains("\nname = \"anvil-brep\"\n"));
    let field = |k: &str| {
        entry
            .and_then(|p| p.lines().find_map(|l| l.strip_prefix(k)))
            .map(|v| v.trim_matches('"').to_string())
    };
    let version = field("version = ").unwrap_or_else(|| "unknown".into());
    let rev = match field("source = ")
        .as_deref()
        .and_then(|s| s.split_once('#'))
    {
        Some((_, sha)) => format!("{version} at {}", &sha[..sha.len().min(7)]),
        None => version,
    };
    println!("cargo:rustc-env=ANVIL_BREP_REV={rev}");
}
