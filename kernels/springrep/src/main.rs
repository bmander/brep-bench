//! The brep-bench runner for springrep (https://crates.io/crates/springrep).
//!
//!     brep-bench-springrep CASE OUT.stl [OUT.step]      (CASE as `boolean/napkin-ring`)
//!     brep-bench-springrep --version
//!     brep-bench-springrep --split [PREFIX]           (each case's build and STEP times, in ms)
//!
//! Builds the case with its own file, `src/cases/<category>/<name_snake>.rs`, then measures the
//! solid with the kernel (exact volume and area, faces by surface, its validity check), meshes it
//! into OUT.stl, writes it as STEP into OUT.step if asked (the kernel's own writer, `write_step`)
//! and prints one line of JSON. Given OUT.step, the STEP is the whole answer: it is written, and
//! nothing is measured or meshed unless the writer declines. A case that refuses, or a kernel
//! operation that declines, is a refusal saying why. The runner never reads the case's description.
use springrep::geom::Surface;
use springrep::mesh::{MeshOptions, tessellate};
use springrep::topo::Brep;
use springrep::{check, measure};
use std::collections::BTreeMap;

mod shared;

/// Every case file, found by `build.rs`.
mod cases {
    include!(concat!(env!("OUT_DIR"), "/cases.rs"));
}

/// A face's surface as the reply counts it.
fn kind(s: &Surface) -> &'static str {
    match s {
        Surface::Plane(..) => "plane",
        Surface::Cylinder(..) => "cylinder",
        Surface::Cone(..) => "cone",
        Surface::Sphere(..) => "sphere",
        Surface::Torus(..) => "torus",
        Surface::Nurbs(..) => "bspline",
        _ => "other",
    }
}

/// A string as a JSON string.
fn json(s: &str) -> String {
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

/// The solid meshed to a sag of a ten-thousandth of its size, or of 1/200 of its thickness
/// (its volume over its area) where that is less — the reference runner's rule
/// (`kernels/occt/run.py`) — written as binary STL.
fn stl(b: &Brep, path: &str, volume: f64, area: f64) -> Result<(), String> {
    let size = measure::bbox(b).diagonal();
    let thickness = if area > 0. && volume > 0. {
        volume / area
    } else {
        size
    };
    let opt = MeshOptions::new((1e-4 * size).min(5e-3 * thickness), 0.1);
    let m = tessellate(b, &opt).map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(84 + 50 * m.triangles.len());
    out.extend_from_slice(&[0u8; 80]);
    out.extend_from_slice(&(m.triangles.len() as u32).to_le_bytes());
    for t in &m.triangles {
        let [a, b, c] = t.map(|i| m.positions[i as usize]);
        let n = (b - a).cross(c - a).unit_or_zero();
        for p in [n.to_array(), a.to_array(), b.to_array(), c.to_array()] {
            for x in p {
                out.extend_from_slice(&(x as f32).to_le_bytes());
            }
        }
        out.extend_from_slice(&[0u8; 2]);
    }
    std::fs::write(path, out).map_err(|e| e.to_string())
}

/// The solid as STEP, by the kernel's own writer, with each edge its curve in space alone: no
/// pcurves, which the standard leaves out as it likes and the harness does not read.
fn write_step(b: &Brep, name: &str) -> Result<String, String> {
    let options = springrep::step::WriteOptions::default().pcurves(false);
    springrep::step::write_with(b, name, &options).map_err(|e| e.to_string())
}

fn refused(why: String) -> String {
    format!("{{\"status\":\"refused\",\"reason\":{}}}", json(&why))
}

/// The reply for a solid built: its measures, once its triangles are written (and its STEP, if
/// asked for). A STEP written is the whole answer: nothing is then measured or meshed.
fn measured(b: &Brep, out: &str, out_step: Option<&str>, case: &str) -> String {
    let step_error = out_step.and_then(|path| {
        write_step(b, case)
            .and_then(|text| std::fs::write(path, text).map_err(|e| e.to_string()))
            .err()
    });
    if out_step.is_some() && step_error.is_none() {
        return "{\"status\":\"ok\"}".into();
    }
    let step_error = step_error
        .map(|e| format!(",\"step_error\":{}", json(&e)))
        .unwrap_or_default();
    let mut faces = BTreeMap::new();
    for f in b.face_ids() {
        *faces.entry(kind(&b.face(f).surface)).or_insert(0) += 1;
    }
    let faces = faces
        .iter()
        .map(|(k, n)| format!("\"{k}\":{n}"))
        .collect::<Vec<_>>()
        .join(",");
    let (a, v) = (measure::area(b), measure::volume(b));
    let valid = check::check(b).is_empty();
    match stl(b, out, v, a) {
        Ok(()) => format!(
            "{{\"status\":\"ok\",\"volume\":{v:e},\"area\":{a:e},\"faces\":{{{faces}}},\"valid\":{valid}{step_error}}}"
        ),
        Err(e) => refused(format!("not meshed: {e}")),
    }
}

/// Under the harness (`BREP_BENCH_WARM` set; SPEC.md, "A runner"), stops the runner until the
/// harness continues it: once it has loaded and found its case, and once it has answered, so that
/// the harness times the kernel alone, between the two.
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn pause() {
    if std::env::var_os("BREP_BENCH_WARM").is_some() {
        use std::io::Write;
        let _ = std::io::stdout().flush();
        unsafe extern "C" {
            fn raise(signal: std::ffi::c_int) -> std::ffi::c_int;
        }
        #[cfg(target_os = "linux")]
        const SIGSTOP: std::ffi::c_int = 19;
        #[cfg(target_os = "macos")]
        const SIGSTOP: std::ffi::c_int = 17;
        // SAFETY: raise only sends a signal to this process
        unsafe { raise(SIGSTOP) };
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn pause() {}

/// Each case whose id starts with `prefix`, built and then written as STEP, each timed: one line
/// a case, its id, the build's milliseconds and the writing's (`-` where it refused or declined).
fn split(prefix: &str) {
    use std::time::Instant;
    let ms = |t: Instant| format!("{:.3}", t.elapsed().as_secs_f64() * 1e3);
    for &(case, build) in cases::CASES.iter().filter(|(id, _)| id.starts_with(prefix)) {
        let t = Instant::now();
        let built = build();
        let build_ms = ms(t);
        let write_ms = match &built {
            Ok(b) => {
                let t = Instant::now();
                match write_step(b, case) {
                    Ok(_) => ms(t),
                    Err(_) => "-".into(),
                }
            }
            Err(_) => "-".into(),
        };
        println!("{case}\t{build_ms}\t{write_ms}");
    }
}

/// The runner's own clock, for the kernel's own time (SPEC.md, "A runner"): started as the
/// runner's first pause ends, and stopped once the kernel has answered, its solid built and
/// written or its refusal in hand. The harness times the same between the two pauses, but a
/// process stopped and continued costs microseconds that are not the kernel's.
struct Clock {
    since: std::time::Instant,
    stopped: std::cell::Cell<Option<f64>>,
}

impl Clock {
    fn start() -> Clock {
        Clock { since: std::time::Instant::now(), stopped: std::cell::Cell::new(None) }
    }

    /// Stops the clock, unless it is stopped: its seconds.
    fn stop(&self) -> f64 {
        let seconds = self.stopped.get().unwrap_or_else(|| self.since.elapsed().as_secs_f64());
        self.stopped.set(Some(seconds));
        seconds
    }
}

/// The reply with the kernel's own time added to it, `seconds`, the clock stopped now if the
/// kernel's refusing did not stop it.
fn timed(reply: &impl std::fmt::Display, clock: &Clock) -> String {
    let seconds = clock.stop();
    let reply = reply.to_string();
    match reply.strip_suffix('}') {
        Some(open) => format!("{open},\"seconds\":{seconds:e}}}"),
        None => reply,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--version") {
        println!(
            "springrep {} (brep-bench runner {})",
            env!("SPRINGREP_REV"),
            env!("CARGO_PKG_VERSION")
        );
        return;
    }
    if args.get(1).map(String::as_str) == Some("--split") {
        split(args.get(2).map_or("", String::as_str));
        return;
    }
    let (case, out, out_step) = match &args[..] {
        [_, case, out] => (case, out, None),
        [_, case, out, step] => (case, out, Some(step.as_str())),
        _ => {
            eprintln!("usage: brep-bench-springrep CASE OUT.stl [OUT.step]");
            std::process::exit(2);
        }
    };
    let Some(&(_, build)) = cases::CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}`");
        std::process::exit(2);
    };
    pause();
    let clock = Clock::start();
    let reply = match build() {
        Ok(solid) => measured(&solid, out, out_step, case),
        Err(why) => {
            clock.stop();
            refused(why)
        }
    };
    println!("{}", timed(&reply, &clock));
    pause();
}
