//! Finds every case file under `src/cases/<category>/<name_snake>.rs` and writes the table the
//! runner looks a case id up in (`$OUT_DIR/cases.rs`): adding a case is adding its file. Also reads
//! which commit of cad-kernel `Cargo.lock` pins, for `--version`.
use std::fmt::Write as _;
use std::path::Path;

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cases = root.join("src/cases");
    println!("cargo:rerun-if-changed=src/cases");
    println!("cargo:rerun-if-changed=Cargo.lock");

    let mut categories: Vec<_> = std::fs::read_dir(&cases).expect("src/cases reads")
        .map(|e| e.expect("a directory entry").path())
        .filter(|p| p.is_dir())
        .collect();
    categories.sort();
    let (mut modules, mut table) = (String::new(), String::new());
    for dir in categories {
        let category = dir.file_name().unwrap().to_str().unwrap().to_string();
        let mut files: Vec<_> = std::fs::read_dir(&dir).expect("a category reads")
            .map(|e| e.expect("a directory entry").path())
            .filter(|p| p.extension().is_some_and(|x| x == "rs"))
            .collect();
        files.sort();
        writeln!(modules, "pub mod r#{category} {{").unwrap();
        for file in files {
            let name = file.file_stem().unwrap().to_str().unwrap().to_string();
            writeln!(modules, "    #[path = {:?}]\n    pub mod r#{name};", file.display().to_string()).unwrap();
            writeln!(table, "    (\"{category}/{}\", r#{category}::r#{name}::build),", name.replace('_', "-")).unwrap();
        }
        writeln!(modules, "}}").unwrap();
    }
    let out = format!("{modules}\n/// Every case by its id, and the function that builds it.\n\
        pub const CASES: &[(&str, fn() -> Result<cad_kernel::topo::Brep, String>)] = &[\n{table}];\n");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    std::fs::write(Path::new(&out_dir).join("cases.rs"), out).unwrap();

    // the commit the lock file pins cad-kernel at: `source = "git+…#<sha>"`
    let lock = std::fs::read_to_string(root.join("Cargo.lock")).unwrap_or_default();
    let rev = lock.split("[[package]]").find(|p| p.contains("name = \"cad-kernel\""))
        .and_then(|p| p.split('#').nth(1)).and_then(|s| s.get(..7)).unwrap_or("unknown");
    println!("cargo:rustc-env=CAD_KERNEL_REV={rev}");
}
