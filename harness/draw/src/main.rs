//! Draw every case: a third-angle sheet of its expected solid (front, top and right views at one
//! scale, and an isometric view), and the explorer's thumbnail of it, as PNGs under
//! site/drawings/<case>/.
//!
//! ```text
//! cargo run --release --manifest-path harness/draw/Cargo.toml                # every case
//! cargo run --release --manifest-path harness/draw/Cargo.toml -- boolean/    # the cases whose id starts with this
//! cargo run --release --manifest-path harness/draw/Cargo.toml -- a/b c/d     # the cases whose id starts with any of these
//! ```
//!
//! The solid drawn is a result that passed: the reference kernel's (kernels.json, `"reference":
//! true`) where it passed, else the first kernel's that did; a kernel whose mesh of it has no
//! triangles is passed over for the next. Its triangles are rasterised with a depth buffer,
//! lightly shaded, and outlined where the depth jumps (silhouettes) or the surface turns sharply
//! (creases), as a technical drawing is (render.rs). The thumbnail is the isometric view again,
//! small, lit hard on a dark ground, so a solid stands out from its ground and its turned faces
//! from each other at the explorer's forty pixels.
//!
//! A case is drawn again only when what its drawing is made from changed: the kernel it is drawn
//! from, that kernel's mesh, or this code. Each drawing's stamp (a hash of the mesh and of the
//! code) is kept in site/drawings/index.json, so a run that adds or changes a kernel redraws only
//! the cases whose drawing that changes, not every case it ran. A case whose drawing fails is said
//! so (a warning, under GitHub Actions) and left undrawn, the others drawn: no drawing holds up
//! the run's results.
//!
//! This is harness/draw.py, which it replaced, in Rust: the same arguments, the same lines said,
//! the same files, and in them the same pixels; and the cases are drawn several at once, a thread
//! to a processor.

mod png;
mod render;
mod stl;

use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::{self, Write};
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::{env, fs, process, thread};

/// The code a drawing is made by, each file by its name in src/: a hash of them is in every
/// drawing's stamp, so a change to any draws every case again.
const SOURCES: [(&str, &[u8]); 4] = [
    ("main.rs", include_bytes!("main.rs")),
    ("png.rs", include_bytes!("png.rs")),
    ("render.rs", include_bytes!("render.rs")),
    ("stl.rs", include_bytes!("stl.rs")),
];

/// The bench this was built in: harness/draw's parent's parent.
fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// What a drawing of a mesh is made from: a hash of the code that draws it and of the mesh.
fn stamp(mesh: &[u8]) -> String {
    let mut h = Sha256::new();
    for (_, source) in SOURCES {
        h.update(Sha256::digest(source));
    }
    h.update(Sha256::digest(mesh));
    hex(&h.finalize())
}

fn read_json(path: &Path) -> Result<Value, String> {
    let data = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_slice(&data).map_err(|e| format!("{}: {e}", path.display()))
}

/// What is drawn from, and where to.
struct Bench {
    root: PathBuf,
    /// The kernels a case is drawn from, the first that passed it: the reference, then the rest
    /// by how many cases they pass.
    order: Vec<String>,
    /// Each kernel's outcome of each case it has a result of.
    results: HashMap<String, HashMap<String, String>>,
    /// The last run's index: each case's drawing, as it was left.
    last: Map<String, Value>,
}

impl Bench {
    fn load(root: &Path) -> Result<Bench, String> {
        let path = root.join("kernels.json");
        let registry = read_json(&path)?;
        let kernels = registry["kernels"]
            .as_array()
            .ok_or_else(|| format!("{}: no list of kernels", path.display()))?;
        let mut names = Vec::new();
        let mut order = Vec::new();
        let mut results = HashMap::new();
        for kernel in kernels {
            let name = kernel["name"]
                .as_str()
                .ok_or_else(|| format!("{}: a kernel with no name", path.display()))?;
            if kernel["reference"].as_bool() == Some(true) {
                order.push(name.to_string());
            }
            let path = root.join("results").join(format!("{name}.json"));
            if !path.exists() {
                continue;
            }
            let rows = read_json(&path)?;
            let rows = rows["cases"]
                .as_array()
                .ok_or_else(|| format!("{}: no list of cases", path.display()))?;
            let mut outcomes = HashMap::new();
            for row in rows {
                match (row["case"].as_str(), row["outcome"].as_str()) {
                    (Some(case), Some(outcome)) => {
                        outcomes.insert(case.to_string(), outcome.to_string());
                    }
                    _ => {
                        return Err(format!(
                            "{}: a result with no case or no outcome",
                            path.display()
                        ));
                    }
                }
            }
            names.push(name.to_string());
            results.insert(name.to_string(), outcomes);
        }
        // the reference first, then the rest by how many cases they pass (those that pass as
        // many, as kernels.json lists them), each once
        let passes = |name: &String| results[name].values().filter(|o| *o == "pass").count();
        names.sort_by_key(|name| std::cmp::Reverse(passes(name)));
        for name in names {
            if !order.contains(&name) {
                order.push(name);
            }
        }
        let path = root.join("site/drawings/index.json");
        let last = match path.exists() {
            true => match read_json(&path)? {
                Value::Object(last) => last,
                _ => return Err(format!("{}: not an index of drawings", path.display())),
            },
            false => Map::new(),
        };
        Ok(Bench {
            root: root.to_path_buf(),
            order,
            results,
            last,
        })
    }

    /// Every case any kernel has a result of, in order.
    fn case_ids(&self) -> BTreeSet<&str> {
        self.results
            .values()
            .flat_map(|r| r.keys().map(String::as_str))
            .collect()
    }

    /// Whether the last drawing of a case is still the one to show: made from the same kernel's
    /// same mesh by the same code, and all there.
    fn current(&self, cid: &str, name: &str, made_from: &str, out_dir: &Path) -> bool {
        self.last.get(cid).is_some_and(|drawn| {
            drawn["from"].as_str() == Some(name)
                && drawn["stamp"].as_str() == Some(made_from)
                && render::DRAWINGS
                    .iter()
                    .all(|view| out_dir.join(format!("{view}.png")).exists())
        })
    }

    /// A case drawn, where there is something to draw it from and its last drawing is not still
    /// the one to show: what was said of it, and what became of it.
    fn draw(&self, cid: &str) -> Done {
        let mut said = Vec::new();
        // the first kernel that passed it with a mesh of some triangles. A mesh of none (a solid
        // Open CASCADE could not mesh) is passed over, and said so, for the next kernel's
        for name in &self.order {
            let passed = self
                .results
                .get(name)
                .and_then(|r| r.get(cid))
                .is_some_and(|outcome| outcome == "pass" || outcome == "built");
            let path = self
                .root
                .join("results/meshes")
                .join(name)
                .join(format!("{cid}.stl"));
            if !passed || !path.exists() {
                continue;
            }
            // a drawing that fails, in whatever way, fails for its case alone
            let drawing =
                panic::catch_unwind(AssertUnwindSafe(|| self.draw_from(cid, name, &path)))
                    .unwrap_or_else(|_| Err("the drawing of it panicked".into()));
            let drawing = match drawing {
                Ok(None) => {
                    said.push(format!(
                        "{cid}: {name}'s mesh has no triangles, not drawn from"
                    ));
                    continue;
                }
                Ok(Some(drawing)) => drawing,
                Err(why) => Drawing::Failed {
                    from: name.clone(),
                    why: format!("{}: {why}", path.display()),
                },
            };
            match drawing {
                Drawing::Drawn(_) => said.push(format!("{cid}: drawn from {name}")),
                Drawing::Failed { .. } => {
                    said.push(format!("{cid}: could not be drawn from {name}"))
                }
                _ => {}
            }
            return Done { said, drawing };
        }
        said.push(format!("{cid}: no passing result with a mesh to draw"));
        Done {
            said,
            drawing: Drawing::Undrawn,
        }
    }

    /// A case's drawings made from a kernel's mesh of it, unless its last are still the ones to
    /// show; nothing, of a mesh with no triangles.
    fn draw_from(&self, cid: &str, name: &str, path: &Path) -> Result<Option<Drawing>, String> {
        let mesh = fs::read(path).map_err(|e| e.to_string())?;
        if !stl::has_triangles(&mesh) {
            return Ok(None);
        }
        let out_dir = self.root.join("site/drawings").join(cid);
        let made_from = stamp(&mesh);
        if self.current(cid, name, &made_from, &out_dir) {
            return Ok(Some(Drawing::Kept));
        }
        let drawings = render::sheet(&stl::read(&mesh)?)?;
        fs::create_dir_all(&out_dir).map_err(|e| format!("{}: {e}", out_dir.display()))?;
        let mut sizes = Map::new();
        for (view, image) in &drawings {
            let to = out_dir.join(format!("{view}.png"));
            fs::write(&to, png::encode(image.width, image.height, &image.rgb))
                .map_err(|e| format!("{}: {e}", to.display()))?;
            sizes.insert(view.to_string(), json!([image.width, image.height]));
        }
        Ok(Some(Drawing::Drawn(
            json!({"from": name, "sizes": sizes, "stamp": made_from}),
        )))
    }
}

/// What became of a case.
enum Drawing {
    /// No kernel passed it with a mesh to draw.
    Undrawn,
    /// Its last drawing is still the one to show.
    Kept,
    /// It was drawn: its entry in the index.
    Drawn(Value),
    /// It could not be drawn from the kernel it was to be drawn from, and why.
    Failed { from: String, why: String },
}

/// A case seen to: the lines to say of it, and what became of it.
struct Done {
    said: Vec<String>,
    drawing: Drawing,
}

/// Draw the cases of the bench at `root` whose id starts with any of `prefixes` (every case,
/// given none), saying on `out` what draw.py said, and on `err` why a case could not be drawn.
fn run(
    root: &Path,
    prefixes: &[String],
    out: &mut impl Write,
    err: &mut impl Write,
) -> Result<(), String> {
    let bench = Bench::load(root)?;
    // no prefix, or the empty one, is every case: the index is then of these cases alone
    let every = prefixes.iter().all(String::is_empty);
    let case_ids = bench.case_ids();
    let cases: Vec<&str> = case_ids
        .iter()
        .copied()
        .filter(|cid| every || prefixes.iter().any(|p| cid.starts_with(p.as_str())))
        .collect();
    let mut index = Map::new();
    let mut undrawn = HashSet::new();
    let mut kept = 0;
    let mut failed = Vec::new();
    // the cases are drawn several at once, each thread taking the next not yet taken, and seen
    // to here in their order as they are done
    let next = AtomicUsize::new(0);
    let threads = thread::available_parallelism()
        .map_or(1, |n| n.get())
        .min(cases.len());
    thread::scope(|scope| {
        let (done, dones) = mpsc::channel();
        for _ in 0..threads {
            let done = done.clone();
            scope.spawn(|| {
                let done = done;
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    if i >= cases.len() || done.send((i, bench.draw(cases[i]))).is_err() {
                        break;
                    }
                }
            });
        }
        drop(done);
        let mut waiting: Vec<Option<Done>> = cases.iter().map(|_| None).collect();
        let mut at = 0;
        for (i, case) in dones {
            waiting[i] = Some(case);
            while let Some(case) = waiting.get_mut(at).and_then(Option::take) {
                let cid = cases[at];
                at += 1;
                for line in case.said {
                    writeln!(out, "{line}").ok();
                }
                match case.drawing {
                    Drawing::Undrawn => {
                        undrawn.insert(cid);
                    }
                    Drawing::Kept => {
                        index.insert(cid.to_string(), bench.last[cid].clone());
                        kept += 1;
                    }
                    Drawing::Drawn(entry) => {
                        index.insert(cid.to_string(), entry);
                    }
                    // a drawing that fails is said so and left undrawn: the run's results, kept
                    // after the drawings, must not wait on them
                    Drawing::Failed { from, why } => {
                        writeln!(err, "draw: {cid}: {why}").ok();
                        failed.push(format!("{cid} ({from})"));
                        undrawn.insert(cid);
                    }
                }
            }
        }
    });
    if !failed.is_empty() {
        // under GitHub Actions, a warning on the run's summary
        let actions = env::var_os("GITHUB_ACTIONS").is_some_and(|v| !v.is_empty());
        let prefix = if actions { "::warning::" } else { "" };
        let (n, cases) = (failed.len(), failed.join(", "));
        writeln!(out, "{prefix}{n} cases could not be drawn: {cases}").ok();
    }
    if kept > 0 {
        writeln!(out, "{kept} drawings unchanged, kept").ok();
    }
    if !every {
        // the others' drawings are kept, but not of a case with nothing to draw any more
        let mut all = bench.last.clone();
        all.extend(index);
        all.retain(|cid, _| case_ids.contains(cid.as_str()) && !undrawn.contains(cid.as_str()));
        index = all;
    }
    let path = root.join("site/drawings/index.json");
    fs::create_dir_all(path.parent().unwrap())
        .and_then(|_| fs::write(&path, indented(&Value::Object(index))))
        .map_err(|e| format!("{}: {e}", path.display()))
}

/// A value as JSON, indented a space a level: as Python's `json.dump(value, indent=1)` writes it.
fn indented(value: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b" ");
    let mut serializer = serde_json::Serializer::with_formatter(&mut out, formatter);
    serde::Serialize::serialize(value, &mut serializer).unwrap();
    out
}

fn main() {
    let prefixes: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = run(
        root(),
        &prefixes,
        &mut io::stdout().lock(),
        &mut io::stderr(),
    ) {
        eprintln!("draw: {e}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicU32;

    /// A bench of its own in the temporary directory, removed when done with.
    struct Temp(PathBuf);

    impl Temp {
        fn new() -> Temp {
            static NEXT: AtomicU32 = AtomicU32::new(0);
            let n = NEXT.fetch_add(1, Ordering::Relaxed);
            let dir = env::temp_dir().join(format!("brep-bench-draw-{}-{n}", process::id()));
            fs::create_dir_all(&dir).unwrap();
            Temp(dir)
        }

        fn write(&self, path: &str, data: impl AsRef<[u8]>) {
            let path = self.0.join(path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, data).unwrap();
        }

        /// The kernels, the first the reference, each with its results: (case, outcome).
        fn kernels(&self, kernels: &[(&str, &[(&str, &str)])]) {
            let registry: Vec<Value> = kernels
                .iter()
                .enumerate()
                .map(|(i, (name, _))| match i {
                    0 => json!({"name": name, "reference": true}),
                    _ => json!({"name": name}),
                })
                .collect();
            self.write("kernels.json", json!({"kernels": registry}).to_string());
            for (name, rows) in kernels {
                let rows: Vec<Value> = rows
                    .iter()
                    .map(|(case, outcome)| json!({"case": case, "outcome": outcome, "ms": 1.5}))
                    .collect();
                self.write(
                    &format!("results/{name}.json"),
                    json!({"cases": rows}).to_string(),
                );
            }
        }

        /// A kernel's mesh of a case: a block so long.
        fn mesh(&self, kernel: &str, case: &str, length: f64) {
            let tris = render::block([0.0; 3], [length, 1.0, 1.0]);
            self.write(
                &format!("results/meshes/{kernel}/{case}.stl"),
                stl::binary(&tris),
            );
        }

        /// A run: what it said, line by line, and why any case could not be drawn.
        fn run_and_why(&self, prefixes: &[&str]) -> Result<(Vec<String>, String), String> {
            let prefixes: Vec<String> = prefixes.iter().map(|p| p.to_string()).collect();
            let (mut out, mut err) = (Vec::new(), Vec::new());
            run(&self.0, &prefixes, &mut out, &mut err)?;
            let said = String::from_utf8(out).unwrap();
            // a warning, where the tests are run under GitHub Actions
            let lines = said
                .lines()
                .map(|line| line.trim_start_matches("::warning::").to_string());
            Ok((lines.collect(), String::from_utf8(err).unwrap()))
        }

        /// A run: what it said, line by line.
        fn run(&self, prefixes: &[&str]) -> Result<Vec<String>, String> {
            self.run_and_why(prefixes).map(|(said, _)| said)
        }

        fn index(&self) -> Map<String, Value> {
            match read_json(&self.0.join("site/drawings/index.json")).unwrap() {
                Value::Object(index) => index,
                _ => panic!("not an index"),
            }
        }

        fn drawn(&self, case: &str) -> bool {
            render::DRAWINGS.iter().all(|view| {
                self.0
                    .join(format!("site/drawings/{case}/{view}.png"))
                    .exists()
            })
        }
    }

    impl Drop for Temp {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).ok();
        }
    }

    /// Every file of src/ is among the sources a drawing's stamp is made from.
    #[test]
    fn the_stamp_is_of_every_source() {
        let mut files: Vec<String> =
            fs::read_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("src"))
                .unwrap()
                .map(|entry| entry.unwrap().file_name().into_string().unwrap())
                .collect();
        files.sort();
        assert_eq!(files, SOURCES.map(|(name, _)| name));
    }

    #[test]
    fn the_stamp_is_of_the_mesh() {
        assert_eq!(stamp(b"a"), stamp(b"a"));
        assert_ne!(stamp(b"a"), stamp(b"b"));
        assert_eq!(stamp(b"a").len(), 64);
    }

    #[test]
    fn a_case_is_drawn_and_then_kept() {
        let bench = Temp::new();
        bench.kernels(&[("ref", &[("a/one", "pass")])]);
        bench.mesh("ref", "a/one", 2.0);
        assert_eq!(bench.run(&[]).unwrap(), ["a/one: drawn from ref"]);
        assert!(bench.drawn("a/one"));
        let index = bench.index();
        assert_eq!(index["a/one"]["from"], "ref");
        assert_eq!(index["a/one"]["sizes"]["thumb"], json!([160, 160]));
        assert_eq!(index["a/one"]["stamp"].as_str().unwrap().len(), 64);
        // as draw.py wrote it: a space a level, the drawings in their order
        let text = fs::read_to_string(bench.0.join("site/drawings/index.json")).unwrap();
        assert!(text.starts_with(
            "{\n \"a/one\": {\n  \"from\": \"ref\",\n  \"sizes\": {\n   \"front\": [\n    "
        ));
        let at = |key: &str| text.find(key).unwrap();
        let keys = [
            "\"front\"",
            "\"top\"",
            "\"right\"",
            "\"iso\"",
            "\"thumb\"",
            "\"stamp\"",
        ];
        assert!(keys.windows(2).all(|pair| at(pair[0]) < at(pair[1])));
        assert!(text.ends_with("\n }\n}"));

        assert_eq!(bench.run(&[]).unwrap(), ["1 drawings unchanged, kept"]);
        assert_eq!(bench.index(), index);
    }

    #[test]
    fn a_case_is_drawn_again_when_what_it_is_made_from_changes() {
        let bench = Temp::new();
        bench.kernels(&[
            ("ref", &[("a/one", "pass")]),
            ("other", &[("a/one", "pass")]),
        ]);
        bench.mesh("ref", "a/one", 2.0);
        bench.mesh("other", "a/one", 2.0);
        bench.run(&[]).unwrap();
        let first = bench.index();
        // its mesh
        bench.mesh("ref", "a/one", 3.0);
        assert_eq!(bench.run(&[]).unwrap(), ["a/one: drawn from ref"]);
        let second = bench.index();
        assert_ne!(second["a/one"]["stamp"], first["a/one"]["stamp"]);
        // the kernel it is drawn from, though the mesh is the same
        bench.mesh("other", "a/one", 3.0);
        bench.kernels(&[
            ("ref", &[("a/one", "fail")]),
            ("other", &[("a/one", "pass")]),
        ]);
        assert_eq!(bench.run(&[]).unwrap(), ["a/one: drawn from other"]);
        assert_eq!(bench.index()["a/one"]["stamp"], second["a/one"]["stamp"]);
        // a drawing gone missing
        fs::remove_file(bench.0.join("site/drawings/a/one/top.png")).unwrap();
        assert_eq!(bench.run(&[]).unwrap(), ["a/one: drawn from other"]);
        assert!(bench.drawn("a/one"));
    }

    /// The reference where it passed, else the kernel that passes the most cases; a result only
    /// built counts, a failure does not.
    #[test]
    fn a_case_is_drawn_from_the_first_kernel_that_passed_it() {
        let bench = Temp::new();
        bench.kernels(&[
            (
                "ref",
                &[("a/one", "pass"), ("a/two", "fail"), ("a/three", "refused")],
            ),
            (
                "few",
                &[("a/one", "pass"), ("a/two", "pass"), ("a/three", "built")],
            ),
            (
                "many",
                &[("a/one", "pass"), ("a/two", "pass"), ("a/four", "pass")],
            ),
        ]);
        for kernel in ["ref", "few", "many"] {
            for case in ["a/one", "a/two", "a/three", "a/four"] {
                bench.mesh(kernel, case, 2.0);
            }
        }
        assert_eq!(
            bench.run(&[]).unwrap(),
            [
                "a/four: drawn from many",
                "a/one: drawn from ref",
                "a/three: drawn from few",
                "a/two: drawn from many",
            ]
        );
        let index = bench.index();
        let cases: Vec<&String> = index.keys().collect();
        assert_eq!(cases, ["a/four", "a/one", "a/three", "a/two"]);
    }

    #[test]
    fn a_mesh_of_no_triangles_is_passed_over() {
        let bench = Temp::new();
        bench.kernels(&[
            (
                "ref",
                &[("a/one", "pass"), ("a/two", "pass"), ("a/three", "pass")],
            ),
            ("other", &[("a/one", "pass"), ("a/two", "pass")]),
        ]);
        bench.write("results/meshes/ref/a/one.stl", stl::binary(&[]));
        bench.mesh("other", "a/one", 2.0);
        bench.write("results/meshes/ref/a/two.stl", "solid s\nendsolid s\n");
        assert_eq!(
            bench.run(&[]).unwrap(),
            [
                "a/one: ref's mesh has no triangles, not drawn from",
                "a/one: drawn from other",
                "a/three: no passing result with a mesh to draw",
                "a/two: ref's mesh has no triangles, not drawn from",
                "a/two: no passing result with a mesh to draw",
            ]
        );
        let index = bench.index();
        assert_eq!(index.keys().collect::<Vec<_>>(), ["a/one"]);
        assert_eq!(index["a/one"]["from"], "other");
    }

    /// Given prefixes, only the cases they name are drawn; the others' drawings are kept in the
    /// index as they were, but not those of a case that is gone or has nothing to draw any more.
    #[test]
    fn prefixes_draw_their_cases_and_keep_the_rest() {
        let bench = Temp::new();
        let all = [
            ("a/one", "pass"),
            ("b/two", "pass"),
            ("b/three", "pass"),
            ("c/gone", "pass"),
        ];
        bench.kernels(&[("ref", &all)]);
        for (case, _) in all {
            bench.mesh("ref", case, 2.0);
        }
        bench.run(&[]).unwrap();
        let first = bench.index();

        // c/gone is removed, d/new added, b/three fails now, and every mesh changes
        let now = [
            ("a/one", "pass"),
            ("b/two", "pass"),
            ("b/three", "fail"),
            ("d/new", "pass"),
        ];
        bench.kernels(&[("ref", &now)]);
        for (case, _) in now {
            bench.mesh("ref", case, 3.0);
        }
        assert_eq!(
            bench.run(&["b/", "d/n"]).unwrap(),
            [
                "b/three: no passing result with a mesh to draw",
                "b/two: drawn from ref",
                "d/new: drawn from ref",
            ]
        );
        let index = bench.index();
        // the last index's order, then the cases new to it
        assert_eq!(
            index.keys().collect::<Vec<_>>(),
            ["a/one", "b/two", "d/new"]
        );
        assert_eq!(index["a/one"], first["a/one"]);
        assert_ne!(index["b/two"], first["b/two"]);

        // the empty prefix is every case
        assert_eq!(
            bench.run(&[""]).unwrap(),
            [
                "a/one: drawn from ref",
                "b/three: no passing result with a mesh to draw",
                "2 drawings unchanged, kept",
            ]
        );
    }

    /// A case whose drawing fails is said so and left undrawn, and the others are drawn: the run
    /// does not fail.
    #[test]
    fn a_case_that_cannot_be_drawn_is_left_undrawn() {
        let bench = Temp::new();
        let cases = [("a/one", "pass"), ("a/three", "pass"), ("a/two", "pass")];
        bench.kernels(&[("ref", &cases), ("other", &cases)]);
        for (case, _) in cases {
            bench.mesh("ref", case, 2.0);
            bench.mesh("other", case, 2.0);
        }
        bench.run(&[]).unwrap();
        assert_eq!(bench.index().len(), 3);

        // the reference's meshes of two now cannot be read: the next kernel's is not drawn instead
        bench.write(
            "results/meshes/ref/a/two.stl",
            "solid s vertex 0 0 0 vertex 1 0 0 endsolid",
        );
        bench.write(
            "results/meshes/ref/a/three.stl",
            "solid s vertex 0 0 nought endsolid",
        );
        let (said, why) = bench.run_and_why(&["a/t"]).unwrap();
        assert_eq!(
            said,
            [
                "a/three: could not be drawn from ref",
                "a/two: could not be drawn from ref",
                "2 cases could not be drawn: a/three (ref), a/two (ref)",
            ]
        );
        let why: Vec<&str> = why.lines().collect();
        assert_eq!(why.len(), 2);
        assert!(
            why[0].starts_with("draw: a/three: ") && why[0].ends_with("no number"),
            "{why:?}"
        );
        assert!(
            why[1].starts_with("draw: a/two: ") && why[1].ends_with("not in threes"),
            "{why:?}"
        );
        // their last drawings are not kept in the index
        assert_eq!(bench.index().keys().collect::<Vec<_>>(), ["a/one"]);

        let (said, _) = bench.run_and_why(&[]).unwrap();
        assert_eq!(
            said,
            [
                "a/three: could not be drawn from ref",
                "a/two: could not be drawn from ref",
                "2 cases could not be drawn: a/three (ref), a/two (ref)",
                "1 drawings unchanged, kept",
            ]
        );
        assert_eq!(bench.index().keys().collect::<Vec<_>>(), ["a/one"]);
    }

    /// What every drawing is made from failing to be read does fail the run.
    #[test]
    fn a_bench_that_cannot_be_read_fails_the_run() {
        let bench = Temp::new();
        assert!(bench.run(&[]).unwrap_err().contains("kernels.json"));
        bench.kernels(&[("ref", &[("a/one", "pass")])]);
        bench.write("results/ref.json", "{");
        assert!(bench.run(&[]).unwrap_err().contains("ref.json"));
    }

    #[test]
    fn a_bench_with_no_results_draws_nothing() {
        let bench = Temp::new();
        bench.kernels(&[]);
        assert_eq!(bench.run(&[]).unwrap(), Vec::<String>::new());
        assert!(bench.index().is_empty());
    }
}
