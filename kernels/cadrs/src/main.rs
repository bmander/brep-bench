//! The brep-bench runner for cadrs's kernel (https://github.com/rvdende/cadrs, the crate
//! `cadrs_kernel`).
//!
//!     brep-bench-cadrs CASE OUT.stl [OUT.step]
//!     brep-bench-cadrs --version
//!
//! Builds the case `CASE` (`boolean/napkin-ring`) with its implementation,
//! `src/cases/boolean/napkin_ring.rs`, in a session of cadrs's native kernel (`DefaultKernel`)
//! through the `Kernel` trait, the API cadrs's Part Studio features call. It then measures the body
//! with the kernel's own tools: its volume and area integrated over the exact B-rep
//! (`Kernel::mass_properties`), its faces by surface kind, and the kernel's full validity check
//! (`Body::check_with(CheckLevel::Full)`). It meshes the body with the kernel's tessellator
//! (`Kernel::tessellate`) into OUT.stl and, if asked, writes it as STEP (AP214) into OUT.step with
//! the kernel's own writer (`Kernel::export_step`; where the writer declines the body, the reply
//! says why in `step_error`), and prints one line of JSON. Given OUT.step, the STEP is the whole
//! answer: it is written, and nothing is measured or meshed unless the writer declines. An
//! operation's error is a refusal: the kernel declined, saying why. A case may also refuse in its
//! own words, where the kernel has no operation for what it describes. The runner never reads the
//! case's description.
use cadrs_kernel::native::{CheckLevel, Surface};
use cadrs_kernel::{BodyClass, BodyId, DefaultKernel, Kernel, KernelError, Tessellation};
use std::collections::BTreeMap;
use std::process::ExitCode;

#[allow(dead_code)]
mod bench;
#[allow(dead_code, unused_imports)]
mod shared;

/// A case's implementation: the body it builds in the session, or why it was not built.
type Build = fn(&mut DefaultKernel) -> Result<BodyId, Refusal>;

/// Why a case was not built: the kernel's error, in its own words, or a case that refuses in its
/// own because the kernel has no operation for what it describes.
#[derive(Debug)]
pub struct Refusal(String);

impl From<KernelError> for Refusal {
    fn from(e: KernelError) -> Self {
        Refusal(e.to_string())
    }
}

impl From<&str> for Refusal {
    fn from(why: &str) -> Self {
        Refusal(why.to_owned())
    }
}

impl From<String> for Refusal {
    fn from(why: String) -> Self {
        Refusal(why)
    }
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// One module per case file, and `CASES`, written by `build.rs`.
mod cases {
    use super::{Build, Refusal};
    include!(concat!(env!("OUT_DIR"), "/cases.rs"));
}

/// A face's surface as the reply counts it.
fn kind(s: &Surface) -> &'static str {
    match s {
        Surface::Plane { .. } => "plane",
        Surface::Cylinder { .. } => "cylinder",
        Surface::Cone { .. } => "cone",
        Surface::Sphere { .. } => "sphere",
        Surface::Torus { .. } => "torus",
        Surface::Nurbs(_) => "bspline",
        _ => "other",
    }
}

/// A JSON string.
fn quoted(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn refused(why: &str) -> String {
    format!("{{\"status\":\"refused\",\"reason\":{}}}", quoted(why))
}

/// The volume the triangles bound (by the divergence theorem), their points rounded by `at`.
fn mesh_volume(mesh: &cadrs_kernel::TriMesh, at: impl Fn(usize) -> [f64; 3]) -> f64 {
    mesh.indices
        .iter()
        .map(|t| {
            let [a, b, c] = t.map(|i| at(i as usize));
            (a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
                + a[2] * (b[0] * c[1] - b[1] * c[0]))
                / 6.0
        })
        .sum()
}

/// The triangles written as STL: binary (single precision) when that keeps every point the
/// mesher made apart from every other and the volume they bound as it was, else ASCII with each
/// coordinate in full (the shortest decimal that reads back as the same double). The tessellator
/// samples each edge once, so the faces either side of it share its points exactly; single
/// precision rounds points a millionth apart at x = 5 together.
fn write_stl(path: &str, mesh: &cadrs_kernel::TriMesh) -> std::io::Result<()> {
    let single = |i: usize| mesh.positions[i].map(|x| x as f32);
    let mut seen = std::collections::HashSet::with_capacity(mesh.positions.len());
    let mut distinct = std::collections::HashSet::with_capacity(mesh.positions.len());
    let mut apart = true;
    for (i, p) in mesh.positions.iter().enumerate() {
        if distinct.insert([p.x.to_bits(), p.y.to_bits(), p.z.to_bits()]) {
            let q = single(i);
            apart &= seen.insert([q.x.to_bits(), q.y.to_bits(), q.z.to_bits()]);
        }
    }
    let rounded = mesh_volume(mesh, |i| single(i).map(f64::from).into());
    let exact = mesh_volume(mesh, |i| mesh.positions[i].into());
    if apart && (rounded - exact).abs() <= 1e-4 * exact.abs() {
        let mut out = Vec::with_capacity(84 + 50 * mesh.indices.len());
        out.extend_from_slice(&[0u8; 80]);
        out.extend_from_slice(&(mesh.indices.len() as u32).to_le_bytes());
        for t in &mesh.indices {
            let [a, b, c] = t.map(|i| mesh.positions[i as usize]);
            let n = (b - a).cross(&(c - a));
            let n = if n.norm() > 0.0 { n.normalize() } else { n };
            for x in [n.x, n.y, n.z, a.x, a.y, a.z, b.x, b.y, b.z, c.x, c.y, c.z] {
                out.extend_from_slice(&(x as f32).to_le_bytes());
            }
            out.extend_from_slice(&[0u8; 2]);
        }
        return std::fs::write(path, out);
    }
    use std::fmt::Write as _;
    let mut out = String::with_capacity(16 + 160 * mesh.indices.len());
    out.push_str("solid cadrs\n");
    for t in &mesh.indices {
        let [a, b, c] = t.map(|i| mesh.positions[i as usize]);
        let n = (b - a).cross(&(c - a));
        let n = if n.norm() > 0.0 { n.normalize() } else { n };
        let _ = writeln!(out, "facet normal {:e} {:e} {:e}\nouter loop", n.x, n.y, n.z);
        for p in [a, b, c] {
            let _ = writeln!(out, "vertex {:e} {:e} {:e}", p.x, p.y, p.z);
        }
        out.push_str("endloop\nendfacet\n");
    }
    out.push_str("endsolid cadrs\n");
    std::fs::write(path, out)
}

/// The reply for a body built: its measures, once its triangles are written. The mesh is the
/// coarsest of a ladder of chords that bounds the body's exact volume to 1/100 (the harness asks
/// 1/50): from a thousandth of the body's size, or a twentieth of its thickness (its volume over
/// its area) where that is less, at 15°, each step a chord where the volume would be off 0.7 of
/// that (in proportion to how far off this one is, a half to a sixteenth of it) and half the angle
/// (to 5°), down to a hundred-thousandth of the size. With `out_step`, the body is also written
/// there as STEP by the kernel's writer, or the reply says why it was not (`step_error`). A STEP
/// written is the whole answer: nothing is then measured or meshed.
fn measured(k: &DefaultKernel, body: BodyId, out: &str, out_step: Option<&str>) -> Result<String, Refusal> {
    let timing = std::env::var_os("BREP_TIMING").is_some();
    let t0 = std::time::Instant::now();
    let lap = |what: &str, t: &mut std::time::Instant| {
        if timing {
            eprintln!("{what}: {:.1} ms", t.elapsed().as_secs_f64() * 1e3);
        }
        *t = std::time::Instant::now();
    };
    let mut t = t0;
    match k.body_class(body)? {
        BodyClass::Solid => {}
        other => return Err(format!("the result is not a solid but {other:?}").into()),
    }
    let step_error = || match out_step.map(|path| (path, k.export_step(&[body]))) {
        None => String::new(),
        Some((path, Ok(step))) => {
            std::fs::write(path, step).expect("the STEP file writes");
            String::new()
        }
        Some((_, Err(e))) => format!(",\"step_error\":{}", quoted(&e.to_string())),
    };
    // given OUT.step, the solid written there is the whole answer (SPEC.md, "A runner"): nothing
    // measured or meshed; only where the writer declines does the runner go on to its triangles
    let step_error = step_error();
    if out_step.is_some() && step_error.is_empty() {
        return Ok("{\"status\":\"ok\"}".into());
    }
    let m = k.mass_properties(body)?;
    let (volume, area) = (m.volume, m.surface_area);
    lap("mass", &mut t);
    let native = k.get(body)?;
    let issues = native.check_with(CheckLevel::Full);
    if std::env::var_os("BREP_CHECK").is_some() {
        for i in issues.iter().take(8) {
            eprintln!("check: {i}");
        }
    }
    let valid = issues.is_empty();
    lap("check", &mut t);
    let mut faces = BTreeMap::new();
    for f in native.faces() {
        *faces.entry(kind(&f.surf)).or_insert(0) += 1;
    }
    let faces = faces.iter().map(|(k, n)| format!("\"{k}\":{n}")).collect::<Vec<_>>().join(",");

    let size = k.bounding_box(body)?.diagonal();
    let thickness = if area > 0.0 && volume > 0.0 { volume / area } else { size };
    let mut deflection = (1e-3 * size).min(5e-2 * thickness).max(1e-10);
    let mut angle = 15f64.to_radians();
    let finest = (1e-5 * size).min(5e-4 * thickness).max(1e-10);
    let mesh = loop {
        let quality = Tessellation { deflection, angle };
        let mesh = k.tessellate(body, quality).map_err(|e| format!("not meshed: {e}"))?;
        let bound = mesh_volume(&mesh, |i| mesh.positions[i].into());
        if timing {
            eprintln!(
                "rung: chord {deflection:.3e} angle {:.1}° {} triangles, volume off {:.2e}",
                angle.to_degrees(),
                mesh.indices.len(),
                (bound - volume).abs() / volume.abs()
            );
        }
        let off = (bound - volume).abs() / volume.abs();
        if off <= 1e-2 || deflection <= finest {
            break mesh;
        }
        // (A mesh's volume is off about in proportion to its chord: the next chord where it
        // would be off 0.7 of the bound, from a half to a sixteenth of this one.)
        deflection = (deflection * (0.7e-2 / off).clamp(1.0 / 16.0, 0.5)).max(finest);
        angle = (angle / 2.0).max(5f64.to_radians());
    };
    lap("tessellate", &mut t);
    if std::env::var_os("BREP_FACES").is_some() {
        let mut per: BTreeMap<u64, usize> = BTreeMap::new();
        for f in &mesh.triangle_faces {
            *per.entry(f.0).or_insert(0) += 1;
        }
        for (f, n) in per {
            let surf = &native.faces()[f as usize].surf;
            let spans = match surf {
                cadrs_kernel::native::Surface::Nurbs(ns) => {
                    let count = |k: &[f64]| k.windows(2).filter(|w| w[1] > w[0]).count();
                    format!(", degrees {}×{}, spans {}×{}", ns.du, ns.dv, count(&ns.ku), count(&ns.kv))
                }
                _ => String::new(),
            };
            eprintln!("face {f} {:?}: {n} triangles{spans}", kind(surf));
        }
    }
    write_stl(out, &mesh).map_err(|e| format!("{out} not written: {e}"))?;
    lap(&format!("write {} triangles", mesh.indices.len()), &mut t);
    Ok(format!(
        "{{\"status\":\"ok\",\"volume\":{volume:e},\"area\":{area:e},\"faces\":{{{faces}}},\"valid\":{valid}{step_error}}}"
    ))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--version") {
        println!(
            "cadrs_kernel {} (brep-bench runner {})",
            env!("KERNEL_VERSION"),
            env!("CARGO_PKG_VERSION")
        );
        return ExitCode::SUCCESS;
    }
    // (Profiling: every case built and measured in turn, `ROUNDS` times, in one process.)
    if let [_, all, out] = &args[..]
        && all == "--all"
    {
        cadrs_kernel::native::keep_meshes(false);
        let rounds = std::env::var("ROUNDS").ok().and_then(|n| n.parse().ok()).unwrap_or(1);
        for _ in 0..rounds {
            for (_, build) in cases::CASES {
                let mut k = DefaultKernel::new();
                if let Ok(body) = build(&mut k) {
                    let _ = measured(&k, body, out, None);
                }
            }
        }
        return ExitCode::SUCCESS;
    }
    let (case, out, out_step) = match &args[..] {
        [_, case, out] => (case, out, None),
        [_, case, out, step] => (case, out, Some(step.as_str())),
        _ => {
            eprintln!("usage: brep-bench-cadrs CASE OUT.stl [OUT.step]");
            return ExitCode::from(2);
        }
    };
    let Some(&(_, build)) = cases::CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}`");
        return ExitCode::from(2);
    };
    // Each body is meshed once: no faces' meshes kept for a next time.
    cadrs_kernel::native::keep_meshes(false);
    // (Profiling: the case built over and over, each time in a new session, to sample.)
    if let Some(n) = std::env::var("BREP_REPEAT_BUILD").ok().and_then(|n| n.parse::<usize>().ok()) {
        for _ in 0..n {
            let _ = build(&mut DefaultKernel::new());
        }
    }
    // (Profiling: built and measured over and over, each time in a new session.)
    if let Some(n) = std::env::var("BREP_REPEAT_FRESH").ok().and_then(|n| n.parse::<usize>().ok()) {
        for _ in 0..n {
            let mut k = DefaultKernel::new();
            if let Ok(body) = build(&mut k) {
                let _ = measured(&k, body, out, None);
            }
        }
    }
    let mut k = DefaultKernel::new();
    let t = std::time::Instant::now();
    let built = build(&mut k);
    if std::env::var_os("BREP_TIMING").is_some() {
        eprintln!("build: {:.1} ms", t.elapsed().as_secs_f64() * 1e3);
    }
    // (Profiling: the measures and the mesh made over and over, to sample.)
    if let (Ok(body), Some(n)) = (&built, std::env::var("BREP_REPEAT").ok().and_then(|n| n.parse::<usize>().ok())) {
        for _ in 1..n {
            let _ = measured(&k, *body, out, None);
        }
    }
    let reply = match built.and_then(|body| measured(&k, body, out, out_step)) {
        Ok(reply) => reply,
        Err(why) => refused(&why.to_string()),
    };
    println!("{reply}");
    ExitCode::SUCCESS
}
