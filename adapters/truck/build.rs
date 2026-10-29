//! Writes the truck revision this adapter is built against (from Cargo.lock) into `TRUCK_REV`.
fn main() {
    println!("cargo:rerun-if-changed=Cargo.lock");
    let lock = std::fs::read_to_string("Cargo.lock").unwrap_or_default();
    let rev = lock
        .split("[[package]]")
        .find(|p| p.contains("name = \"truck-modeling\""))
        .and_then(|p| p.lines().find(|l| l.starts_with("source = ")))
        .and_then(|l| l.rsplit('#').next())
        .map(|s| s.trim_end_matches('"').to_string())
        .unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=TRUCK_REV={rev}");
}
