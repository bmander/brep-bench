//! The mass properties of the solid in a STEP file (ISO 10303-21), from its exact geometry: its
//! volume, its area, its centre of gravity and its moments of inertia. No crates.
//!
//!     mass FILE.step     # prints them as JSON
//!
//! The harness grades a kernel on the STEP file it writes (SPEC.md, "STEP is the answer"), so the
//! file's own solid is measured here, by the harness, the same way for every kernel, and not by
//! any one kernel's reader. harness/run.py's `mass_of` runs it, and builds it first where it is not
//! built or is older than its sources
//! (`cargo build --release --manifest-path harness/mass/Cargo.toml`).
//!
//! How. By the divergence theorem each measure of the solid is an integral over its boundary of a
//! polynomial in the position times the outward normal: the volume is ∮ (x·n)/3 dA, its first
//! moment ∫ x dV is ∮ (x²/2) n_x dA, its second moment ∫ x² dV is ∮ (x³/3) n_x dA, and so on. Each
//! face is a surface S(u, v) trimmed by loops of edges, and its integral ∬ F(S, S_u × S_v) du dv
//! over the trimmed region of the (u, v) plane is, by Green's theorem, -∮ H du round the loops,
//! where H(u, v) = ∫ F(u, s) ds from a fixed v_ref to v. Both integrals are taken by Gauss
//! quadrature, the one round the loops adaptively (Gauss-Kronrod), to about 1e-12 of the solid's
//! size.
//!
//! The loops are put on the (u, v) plane by inverting the points of each edge's curve in space
//! onto the face's surface (closed forms for the analytic surfaces, Newton's method on a
//! B-spline), never by the file's own curves in (u, v), which many writers leave out and some
//! write loosely. A periodic parameter (an angle) is unwrapped along each loop, so that a loop
//! round a cylinder or a torus is followed as it winds. Where an edge passes through a singular
//! point of its surface (the pole of a sphere, the apex of a cone) the loop runs along the
//! singular line in (u, v) from the direction it arrives in to the one it leaves in. What a
//! periodic face's loops leave open (whether a face round a sphere holds the pole) is settled by
//! the face's area in (u, v), which must be positive and no more than the whole surface's.
//!
//! The integral round a loop reads H on those inverted points, and takes the path's rate from
//! the curve's direction resolved along the surface's two rates S_u and S_v. The two are the same
//! path only where the surface lets a direction be resolved. Along a fold, where the rates run
//! together (an envelope's sheets, where a swept body's faces are trimmed), they are not, and
//! the integral is taken along the inverted points alone, by chords; by a corner where one rate
//! dies away, or an edge of the surface's domain the curve stands outside, the rate's integral
//! is mended by how far the two paths part (face.rs, `adaptive`). A point of such a fold that
//! Newton's method cannot settle on is found by damped steps (surface.rs, `damped`). A loop
//! whose inverted path breaks between two different points of its surface (an edge laid across
//! a place where the surface meets itself) is said, as an error.
//!
//! A face's loops are taken as ISO 10303-42 directs them: anticlockwise seen from where the face's
//! normal points, the face's normal its surface's turned by `same_sense`. Where a face's normal is
//! against its surface's, the answer also has `by_surface`: the solid as it would be with the
//! loops directed by the surfaces' normals, as some writers direct them; the harness reads it only
//! to say why a file fails. Where a loop along a seam (an edge run both ways, a turn apart) runs
//! against the standard, so that the face is the rest of its surface, which the seam is not in, the
//! answer also has `by_seam`: the solid with those faces the regions their loops unwrap to.
//!
//! A face whose loops, so taken, bound no region of its surface (they run clockwise about its
//! normal, on a surface they do not close round) is the rest of its surface, which no edge ends:
//! the file holds no solid, and the answer is {"invalid": why}, as for a loop that is not
//! closed. That is told from the loops alone, before anything is measured, and only where it is
//! plain: their area is against the standard by far more than it is in doubt, and every sample
//! of their paths lies on the surface where its (u, v) says. It is said at once on a line of its
//! own, since measuring can outlast the time the harness gives it, and the harness reads what
//! was last said; and it is the answer whatever the measuring then comes to. A face that only
//! its measuring finds to bound nothing may be one whose edges were not followed on its surface
//! (one that folds onto itself): of that the answer is still an error, that it cannot be
//! measured.
//!
//! Supported: planes, cylinders, cones, spheres, tori (degenerate too), B-spline surfaces
//! (rational too), surfaces of revolution and of linear extrusion; lines, circles, ellipses,
//! B-spline curves (rational too), polylines and trimmed curves; MANIFOLD_SOLID_BREP,
//! BREP_WITH_VOIDS and FACETED_BREP; lengths in any unit, given in millimetres. Anything else is
//! said, as an error.
//!
//! It was first written in Python (harness/mass.py, since removed) and then in Rust step for step,
//! and still gives the Python measurer's answers digit for digit wherever a loop's two paths keep
//! together (above; they part on about one file in a hundred, which it measured no nearer than
//! they parted): it keeps Python's floored remainder, rounding, hypot and printing (py.rs), and
//! sums in the same order. A change that alters an answer's last digits is a change to how every
//! kernel is measured.
//!
//! The faces are measured several at once (`MASS_THREADS`, by default as many as the machine has
//! cores, eight at most) and summed in the file's order, so the answer does not depend on how
//! many.
#![allow(clippy::needless_range_loop)]

mod curve;
mod face;
mod geom;
mod loops;
mod model;
mod py;
mod step;
mod surface;

use curve::Frame;
use face::{Face, Settled, UNBOUNDED};
use geom::*;
use loops::{Loop, face_loops, newell};
use model::Model;
use py::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use step::Val;
use surface::Surface;

const SOLIDS: [&str; 3] = ["MANIFOLD_SOLID_BREP", "BREP_WITH_VOIDS", "FACETED_BREP"];

/// What `json.dumps` writes.
enum Json {
    Num(f64),
    Int(usize),
    Str(String),
    List(Vec<Json>),
    Obj(Vec<(&'static str, Json)>),
}

impl Json {
    fn dump(&self, out: &mut String) {
        match self {
            Json::Num(x) => out.push_str(&float_json(*x)),
            Json::Int(n) => out.push_str(&n.to_string()),
            Json::Str(s) => out.push_str(&str_json(s)),
            Json::List(v) => {
                out.push('[');
                for (i, x) in v.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    x.dump(out);
                }
                out.push(']');
            }
            Json::Obj(v) => {
                out.push('{');
                for (i, (k, x)) in v.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(&str_json(k));
                    out.push_str(": ");
                    x.dump(out);
                }
                out.push('}');
            }
        }
    }
}

/// Each face of the file's solids: (face entity, its shell's orientation); and how many solids.
fn faces_of(m: &Model) -> R<(Vec<(Val, bool)>, usize)> {
    let mut out = Vec::new();
    let solids: Vec<u64> = m
        .entities
        .list
        .iter()
        .filter(|e| e.names.iter().any(|n| SOLIDS.contains(&n.as_str())))
        .map(|e| e.id)
        .collect();
    if solids.is_empty() {
        return unsupported("no solid in the file");
    }
    for r in &solids {
        let a = m.args_of(*r)?;
        let Some((name, args)) = a.iter().find(|(n, _)| SOLIDS.contains(&n.as_str())) else {
            return Err(Err::Uncaught("StopIteration", String::new()));
        };
        let mut shells = vec![(args.at(1)?.clone(), true)];
        if name == "BREP_WITH_VOIDS" {
            for v in args.at(2)?.items()? {
                shells.push((v.clone(), true));
            }
        }
        for (sh, mut sense) in shells {
            let mut sa = m.one(&sh)?;
            if sa.is("ORIENTED_CLOSED_SHELL") {
                sense = sense == !sa.at(3)?.is("F");
                sa = m.one(sa.at(2)?)?;
            }
            if !(sa.is("CLOSED_SHELL") || sa.is("OPEN_SHELL")) {
                return unsupported(format!("a shell {}", sa.named()));
            }
            for f in sa.at(1)?.items()? {
                out.push((f.clone(), sense));
            }
        }
    }
    Ok((out, solids.len()))
}

/// The middle and the size of a set of points' box.
fn extent(pts: &[V3]) -> (V3, V3) {
    let (mut lo, mut hi) = (pts[0], pts[0]);
    for p in &pts[1..] {
        for i in 0..3 {
            lo[i] = pymin(lo[i], p[i]);
            hi[i] = pymax(hi[i], p[i]);
        }
    }
    (lo, hi)
}

fn bounding(m: &Model) -> R<(V3, f64)> {
    // every point in space: vertices, centres, control points (not the (u, v) points of curves
    // on surfaces, which have two coordinates)
    let mut pts = Vec::new();
    for e in &m.entities.list {
        // (an entity of no name at all has no first name: Python's IndexError)
        let Some(name) = e.names.first() else {
            return index_error();
        };
        if name == "CARTESIAN_POINT" {
            let r = Val::Ref(e.id);
            if m.one(&r)?.at(1)?.len()? == 3 {
                pts.push(m.point(&r)?);
            }
        }
    }
    if pts.is_empty() {
        return unsupported("no points");
    }
    let (lo, hi) = extent(&pts);
    Ok((mul(add(lo, hi), 0.5), pymax(norm(sub(hi, lo)), 1e-9)))
}

/// The error with the face it is of, as `raise type(e)(f"face #{f}: {e}")` gives it.
fn of_face(e: Err, f: &Val) -> Err {
    match e {
        Err::Unsupported(s) => Err::Unsupported(format!("face #{}: {}", f.str(), s)),
        Err::Invalid(s) => Err::Invalid(format!("face #{}: {}", f.str(), s)),
        e => e,
    }
}

struct Found {
    f: Val,
    s: Arc<Surface>,
    loops: Vec<Loop>,
    same: bool,
    sense: bool,
}

/// A face's surface and its loops laid on it.
fn laid(m: &Model, f: &Val, sense: bool, scale: f64) -> R<Found> {
    let fa = m.one(f)?;
    let (s, same);
    if fa.is("ADVANCED_FACE") || fa.is("FACE_SURFACE") {
        s = m.surface(fa.at(2)?)?;
        same = !fa.at(3)?.is("F");
    } else if fa.is("FACE") {
        // a faceted B-rep's polygon: on the plane its points span
        let bounds = fa.at(1)?;
        let la = m.one(m.one(bounds.at(0)?)?.at(1)?)?;
        let mut pts = Vec::new();
        for p in la.at(1)?.items()? {
            pts.push(m.point(p)?);
        }
        let z = unit(newell(&pts))?;
        if pts.len() < 2 {
            return index_error();
        }
        let x = unit(sub(pts[1], pts[0]))?;
        let x = unit(sub(x, mul(z, dot(x, z))))?;
        s = Arc::new(Surface::plane(Frame {
            o: pts[0],
            x,
            y: cross(z, x),
            z,
        }));
        same = true;
    } else {
        return unsupported(format!("a face {}", fa.named()));
    }
    let loops = face_loops(m, f, &s, scale).map_err(|e| of_face(e, f))?;
    Ok(Found {
        f: f.clone(),
        s,
        loops,
        same,
        sense,
    })
}

/// The centre of gravity and the inertia tensor about it, of unit density, from a solid's
/// volume and its first and second moments about `origin` (harness/stl.py's `about_centroid`).
fn about_centroid(
    volume: f64,
    m1: [f64; 3],
    m2: [[f64; 3]; 3],
    origin: V3,
) -> ([f64; 3], [[f64; 3]; 3]) {
    let g = [m1[0] / volume, m1[1] / volume, m1[2] / volume];
    let mut s2 = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            s2[i][j] = m2[i][j] - volume * g[i] * g[j];
        }
    }
    let tr = s2[0][0] + s2[1][1] + s2[2][2];
    let mut inertia = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            inertia[i][j] = if i == j { tr - s2[i][i] } else { -s2[i][j] };
        }
    }
    (
        [origin[0] + g[0], origin[1] + g[1], origin[2] + g[2]],
        inertia,
    )
}

/// The mass properties from a solid's summed integrals, or {"error": why}.
fn props(
    total: &[f64; NM],
    failed: Option<&String>,
    centre: V3,
    nsolids: usize,
    nfaces: usize,
) -> Vec<(&'static str, Json)> {
    if let Some(failed) = failed {
        return vec![("error", Json::Str(format!("cannot measure: {}", failed)))];
    }
    let v = total[2];
    if v == 0.0 {
        return vec![(
            "error",
            Json::Str("cannot measure: the solid has no volume".into()),
        )];
    }
    let m2 = [
        [total[6], total[9], total[11]],
        [total[9], total[7], total[10]],
        [total[11], total[10], total[8]],
    ];
    let (g, inertia) = about_centroid(v, [total[3], total[4], total[5]], m2, centre);
    vec![
        ("volume", Json::Num(v)),
        ("area", Json::Num(total[1])),
        (
            "centroid",
            Json::List(g.iter().map(|x| Json::Num(*x)).collect()),
        ),
        (
            "inertia",
            Json::List(
                inertia
                    .iter()
                    .map(|r| Json::List(r.iter().map(|x| Json::Num(*x)).collect()))
                    .collect(),
            ),
        ),
        ("solids", Json::Int(nsolids)),
        ("faces", Json::Int(nfaces)),
    ]
}

type Measured = R<(Settled, Option<Settled>, Option<Settled>)>;

/// Each face measured, `threads` at once, in the faces' order. A face after one that could not
/// be measured may be left out (`None`): Python stops at the first.
fn measure_faces(
    jobs: Vec<(Arc<Surface>, Vec<Loop>, bool)>,
    centre: V3,
    scale: f64,
    threads: usize,
) -> Vec<Option<Measured>> {
    let n = jobs.len();
    let threads = threads.clamp(1, n.max(1));
    if threads == 1 {
        let failed = AtomicUsize::new(usize::MAX);
        let mut out = Vec::with_capacity(n);
        for (s, loops, same) in jobs {
            let got = Face::measure(s, loops, same, centre, scale, out.len(), &failed);
            let stop = got.is_err();
            out.push(Some(got));
            if stop {
                break;
            }
        }
        out.resize_with(n, || None);
        return out;
    }
    let jobs: Vec<Mutex<Option<(Arc<Surface>, Vec<Loop>, bool)>>> =
        jobs.into_iter().map(|j| Mutex::new(Some(j))).collect();
    let results: Vec<Mutex<Option<Measured>>> = (0..n).map(|_| Mutex::new(None)).collect();
    let next = AtomicUsize::new(0);
    let failed = AtomicUsize::new(usize::MAX);
    let work = || {
        loop {
            let k = next.fetch_add(1, Ordering::SeqCst);
            if k >= n {
                break;
            }
            if k > failed.load(Ordering::SeqCst) {
                continue;
            }
            let (s, loops, same) = jobs[k].lock().unwrap().take().unwrap();
            let got = Face::measure(s, loops, same, centre, scale, k, &failed);
            if got.is_err() {
                failed.fetch_min(k, Ordering::SeqCst);
            }
            *results[k].lock().unwrap() = Some(got);
        }
    };
    std::thread::scope(|scope| {
        // this thread measures too; the others are given its stack's size (a thread that cannot
        // be started is done without)
        for _ in 1..threads {
            let _ = std::thread::Builder::new()
                .stack_size(8 << 20)
                .spawn_scoped(scope, work);
        }
        work();
    });
    results
        .into_iter()
        .map(|r| r.into_inner().unwrap())
        .collect()
}

/// The mass properties of the solid in a STEP file's text: {volume, area, centroid, inertia
/// (about the centroid), solids, faces}, lengths in millimetres, or {"error": why}, or
/// {"invalid": why} where a face's loops plainly bound no region of its surface
/// (`Face::unbounded`); and, where it differs, `by_surface`. Such a face is told to `unsound`
/// before any face is measured (the module's own comment says why).
fn measure(text: &[u8], threads: usize, unsound: &mut dyn FnMut(&str)) -> R<Json> {
    let m = Model::new(text)?;
    let (faces, nsolids) = faces_of(&m)?;
    let (mut centre, mut scale) = bounding(&m)?;
    // each face's loops on its surface first. Where a face cannot be laid on its surface, the
    // file is not measured, and the first such face is why; but the faces after it are still
    // laid, to be looked at below: one that plainly bounds nothing makes the file no solid,
    // whatever is not read of another. A loop that is not closed makes it none at once
    let mut found: Vec<Found> = Vec::with_capacity(faces.len());
    let mut unread: Option<Err> = None;
    for (f, sense) in &faces {
        match laid(&m, f, *sense, scale) {
            Ok(fd) => found.push(fd),
            Err(e @ Err::Invalid(_)) => return Err(e),
            Err(e @ Err::Uncaught(..)) if unread.is_none() => return Err(e),
            Err(e) => {
                unread.get_or_insert(e);
            }
        }
    }
    // the measures about the middle of the solid's own edges, their tolerances by its size: the
    // file's other points (a sphere's centre, a placement's origin) may lie far off a small solid
    // whose measures are the small differences of its faces'
    let mut edge = Vec::new();
    for fd in &found {
        for (ps, _) in &fd.loops {
            for p in ps.iter().filter(|p| p.line.is_none()) {
                let c = p.c.as_ref().unwrap();
                for (t, _) in p.rf.iter().step_by(4) {
                    edge.push(c.point(*t)?);
                }
            }
        }
    }
    if !edge.is_empty() && found.iter().all(|fd| !fd.loops.is_empty()) {
        let (lo, hi) = extent(&edge);
        if norm(sub(hi, lo)) > 0.0 {
            (centre, scale) = (mul(add(lo, hi), 0.5), norm(sub(hi, lo)));
        }
    }
    drop(m);
    // whether every face's loops bound a region of its surface, from the loops alone: the first
    // that plainly do not (a face this cannot tell of, or cannot read, is left to be measured)
    let plain = found
        .iter()
        .find(|fd| Face::unbounded(&fd.s, &fd.loops, fd.same, scale).unwrap_or(false))
        .map(|fd| format!("face #{}: {}", fd.f.str(), UNBOUNDED));
    if let Some(why) = &plain {
        unsound(why);
    }
    let nfaces = faces.len();
    let rest = match unread {
        Some(e) => Err(e),
        None => integrate(found, centre, scale, threads, nsolids, nfaces),
    };
    match (plain, rest) {
        (None, rest) => rest.map(Json::Obj),
        // no solid, whatever measuring it came to: what is kept of that is the solid read
        // another way, which says why (`by_surface`, `by_seam`)
        (Some(why), Ok(out)) => {
            let other = out.into_iter().filter(|(key, _)| key.starts_with("by_"));
            let said = std::iter::once(("invalid", Json::Str(why)));
            Ok(Json::Obj(said.chain(other).collect()))
        }
        // nor is it one where another face could not be read or measured
        (Some(why), Err(Err::Unsupported(_) | Err::Py(..))) => Err(Err::Invalid(why)),
        (Some(_), Err(e)) => Err(e),
    }
}

/// The faces measured and summed: the solid's mass properties as the standard reads it, or
/// {"error": why} (a face whose loops only measuring finds to bound nothing: it may not have
/// followed them on a surface that folds onto itself, so that is not said to be no solid); and
/// `by_surface` and `by_seam`, where they differ.
fn integrate(
    found: Vec<Found>,
    centre: V3,
    scale: f64,
    threads: usize,
    nsolids: usize,
    nfaces: usize,
) -> R<Vec<(&'static str, Json)>> {
    // the solid as the standard reads it, and as a writer who directs a face's loops by its
    // surface's normal would mean it (where any face's normal is against its surface's)
    let against = found.iter().any(|fd| !fd.same);
    let mut names = Vec::with_capacity(found.len());
    let mut jobs = Vec::with_capacity(found.len());
    for fd in found {
        names.push((fd.f, fd.sense));
        jobs.push((fd.s, fd.loops, fd.same));
    }
    let measured = measure_faces(jobs, centre, scale, threads);
    let mut totals = [[0.0; NM]; 3];
    let mut failed: [Option<String>; 3] = [None, None, None];
    let mut by_seam = false;
    for ((f, sense), got) in names.iter().zip(measured) {
        let (standard, other, seamed) = match got {
            Some(Ok(parts)) => parts,
            Some(Err(e)) => return Err(of_face(e, f)),
            None => unreachable!("a face left unmeasured before any failed"),
        };
        let sign = if *sense { 1.0 } else { -1.0 };
        let other = other.unwrap_or_else(|| standard.clone());
        by_seam |= seamed.is_some();
        let seamed = seamed.unwrap_or_else(|| standard.clone());
        for (key, part) in [standard, other, seamed].into_iter().enumerate() {
            let part = match part {
                Ok(part) => part,
                Err(why) => {
                    if failed[key].is_none() {
                        failed[key] = Some(format!("face #{}: {}", f.str(), why));
                    }
                    continue;
                }
            };
            let total = &mut totals[key];
            total[1] += part[1];
            for i in 2..NM {
                total[i] += sign * part[i];
            }
        }
    }
    let mut out = props(&totals[0], failed[0].as_ref(), centre, nsolids, nfaces);
    if against {
        out.push((
            "by_surface",
            Json::Obj(props(
                &totals[1],
                failed[1].as_ref(),
                centre,
                nsolids,
                nfaces,
            )),
        ));
    }
    if by_seam {
        out.push((
            "by_seam",
            Json::Obj(props(
                &totals[2],
                failed[2].as_ref(),
                centre,
                nsolids,
                nfaces,
            )),
        ));
    }
    Ok(out)
}

/// The file as Python reads its text: every line's end a `\n`.
fn newlines(data: Vec<u8>) -> Vec<u8> {
    if !data.contains(&b'\r') {
        return data;
    }
    let mut out = Vec::with_capacity(data.len());
    let mut i = 0;
    while i < data.len() {
        if data[i] == b'\r' {
            out.push(b'\n');
            if data.get(i + 1) == Some(&b'\n') {
                i += 1;
            }
        } else {
            out.push(data[i]);
        }
        i += 1;
    }
    out
}

/// Python's last word, where it would end with a traceback: no answer, and failure.
fn uncaught(kind: &str, why: &str) -> ! {
    eprintln!("Traceback (most recent call last):");
    if why.is_empty() {
        eprintln!("{}", kind);
    } else {
        eprintln!("{}: {}", kind, why);
    }
    std::process::exit(1);
}

/// `measure` of the STEP file at `path`, or {"invalid": why} where the file holds no solid
/// (whoever reads it), or {"error": why} where the measurer cannot measure it.
fn read(path: &str, threads: usize) -> Json {
    let data = match std::fs::read(path) {
        Ok(d) => d,
        Err(e) => {
            let (kind, errno, what) = match e.kind() {
                std::io::ErrorKind::NotFound => {
                    ("FileNotFoundError", 2, "No such file or directory")
                }
                std::io::ErrorKind::PermissionDenied => {
                    ("PermissionError", 13, "Permission denied")
                }
                std::io::ErrorKind::IsADirectory => ("IsADirectoryError", 21, "Is a directory"),
                _ => uncaught("OSError", &format!("{}: {}", e, str_repr(path))),
            };
            uncaught(
                kind,
                &format!("[Errno {}] {}: {}", errno, what, str_repr(path)),
            );
        }
    };
    // (a file found to be no solid is said to be at once, before its answer: see the module)
    match measure(&newlines(data), threads, &mut |why| {
        say(&invalid(why.into()))
    }) {
        Ok(j) => j,
        Err(Err::Invalid(why)) => invalid(why),
        Err(Err::Unsupported(why)) => Json::Obj(vec![(
            "error",
            Json::Str(format!("cannot measure: {}", why)),
        )]),
        Err(Err::Py(kind, why)) => {
            Json::Obj(vec![("error", Json::Str(format!("{}: {}", kind, why)))])
        }
        Err(Err::Uncaught(kind, why)) => uncaught(kind, &why),
    }
}

/// {"invalid": why}: the file holds no solid.
fn invalid(why: String) -> Json {
    Json::Obj(vec![("invalid", Json::Str(why))])
}

/// A line of the answer, written out at once.
fn say(answer: &Json) {
    use std::io::Write;
    let mut out = String::new();
    answer.dump(&mut out);
    println!("{}", out);
    let _ = std::io::stdout().flush();
}

fn main() {
    let threads = match std::env::var("MASS_THREADS")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
    {
        Some(n) => n.max(1),
        None => std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
            .min(8),
    };
    for p in std::env::args().skip(1) {
        say(&read(&p, threads));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A unit cube's corners, and its faces by their corners, each anticlockwise from outside.
    const CORNERS: [V3; 8] = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [1.0, 1.0, 1.0],
        [0.0, 1.0, 1.0],
    ];
    const SIDES: [[usize; 4]; 6] = [
        [0, 3, 2, 1],
        [4, 5, 6, 7],
        [0, 1, 5, 4],
        [1, 2, 6, 5],
        [2, 3, 7, 6],
        [3, 0, 4, 7],
    ];

    /// `measure`'s answer of a file's text, as it is printed, and what it said beforehand.
    fn answer(text: &str, threads: usize) -> (String, Vec<String>) {
        let (mut said, mut out) = (Vec::new(), String::new());
        let unsound = &mut |why: &str| said.push(why.to_string());
        let got = measure(text.as_bytes(), threads, unsound);
        got.ok().unwrap().dump(&mut out);
        (out, said)
    }

    /// The number after `key` in an answer.
    fn number(out: &str, key: &str) -> f64 {
        let at = out.find(key).unwrap() + key.len();
        let end = out[at..].find([',', '}']).unwrap();
        out[at..at + end].trim().parse().unwrap()
    }

    /// A unit cube as a faceted B-rep, in metres: a thousand millimetres a side.
    #[test]
    fn a_faceted_cube() {
        let mut s = String::from("ISO-10303-21;\nHEADER;\nENDSEC;\nDATA;\n");
        s += "#1=(LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT($,.METRE.));\n";
        for (i, p) in CORNERS.iter().enumerate() {
            s += &format!(
                "#{}=CARTESIAN_POINT('',({:?},{:?},{:?}));\n",
                10 + i,
                p[0],
                p[1],
                p[2]
            );
        }
        for (i, f) in SIDES.iter().enumerate() {
            s += &format!(
                "#{}=POLY_LOOP('',(#{},#{},#{},#{}));\n",
                20 + i,
                10 + f[0],
                10 + f[1],
                10 + f[2],
                10 + f[3]
            );
            s += &format!("#{}=FACE_OUTER_BOUND('',#{},.T.);\n", 30 + i, 20 + i);
            s += &format!("#{}=FACE('',(#{}));\n", 40 + i, 30 + i);
        }
        s += "#50=CLOSED_SHELL('',(#40,#41,#42,#43,#44,#45));\n#51=FACETED_BREP('',#50);\nENDSEC;\nEND-ISO-10303-21;\n";
        for threads in [1, 3] {
            let (out, _) = answer(&s, threads);
            let (volume, area) = (number(&out, "\"volume\": "), number(&out, "\"area\": "));
            assert!((volume / 1e9 - 1.0).abs() < 1e-12, "{}", out);
            assert!((area / 6e6 - 1.0).abs() < 1e-12, "{}", out);
            assert!(
                out.contains("\"centroid\": [500.0, 500.0, 500.0]"),
                "{}",
                out
            );
            assert!(out.ends_with("\"solids\": 1, \"faces\": 6}"), "{}", out);
        }
    }

    /// A unit cube of advanced faces, each a plane bounded by four lines; with `wound` false, its
    /// bottom's loop runs the other way round (clockwise about the face's normal).
    fn a_cube(wound: bool) -> String {
        let (pts, faces) = (CORNERS, SIDES);
        let mut s = String::from("ISO-10303-21;\nHEADER;\nENDSEC;\nDATA;\n");
        s += "#1=(LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.));\n";
        let triple = |v: V3| format!("({:?},{:?},{:?})", v[0], v[1], v[2]);
        for (i, p) in pts.iter().enumerate() {
            s += &format!("#{}=CARTESIAN_POINT('',{});\n", 10 + i, triple(*p));
            s += &format!("#{}=VERTEX_POINT('',#{});\n", 20 + i, 10 + i);
        }
        // an edge from the lower of its two vertices to the higher, once
        let mut edges: Vec<(usize, usize)> = Vec::new();
        let mut n = 100;
        let mut shell = Vec::new();
        for f in faces {
            let corners: Vec<V3> = f.iter().map(|&i| pts[i]).collect();
            let z = unit(newell(&corners)).ok().unwrap();
            let x = unit(sub(corners[1], corners[0])).ok().unwrap();
            let mut f = f;
            if !wound && shell.is_empty() {
                f.reverse();
            }
            let mut oriented = Vec::new();
            for k in 0..4 {
                let (a, b) = (f[k], f[(k + 1) % 4]);
                let (lo, hi) = (a.min(b), a.max(b));
                let e = match edges.iter().position(|e| *e == (lo, hi)) {
                    Some(e) => e,
                    None => {
                        let d = sub(pts[hi], pts[lo]);
                        s += &format!("#{}=DIRECTION('',{});\n", n, triple(d));
                        s += &format!("#{}=VECTOR('',#{},1.);\n", n + 1, n);
                        s += &format!("#{}=LINE('',#{},#{});\n", n + 2, 10 + lo, n + 1);
                        s += &format!(
                            "#{}=EDGE_CURVE('',#{},#{},#{},.T.);\n",
                            500 + edges.len(),
                            20 + lo,
                            20 + hi,
                            n + 2
                        );
                        n += 3;
                        edges.push((lo, hi));
                        edges.len() - 1
                    }
                };
                let sense = if a == lo { 'T' } else { 'F' };
                s += &format!("#{}=ORIENTED_EDGE('',*,*,#{},.{}.);\n", n, 500 + e, sense);
                oriented.push(format!("#{}", n));
                n += 1;
            }
            s += &format!("#{}=DIRECTION('',{});\n", n, triple(z));
            s += &format!("#{}=DIRECTION('',{});\n", n + 1, triple(x));
            s += &format!(
                "#{}=AXIS2_PLACEMENT_3D('',#{},#{},#{});\n",
                n + 2,
                10 + f[0],
                n,
                n + 1
            );
            s += &format!("#{}=PLANE('',#{});\n", n + 3, n + 2);
            s += &format!("#{}=EDGE_LOOP('',({}));\n", n + 4, oriented.join(","));
            s += &format!("#{}=FACE_OUTER_BOUND('',#{},.T.);\n", n + 5, n + 4);
            s += &format!(
                "#{}=ADVANCED_FACE('',(#{}),#{},.T.);\n",
                n + 6,
                n + 5,
                n + 3
            );
            shell.push(format!("#{}", n + 6));
            n += 7;
        }
        s += &format!("#900=CLOSED_SHELL('',({}));\n", shell.join(","));
        s + "#901=MANIFOLD_SOLID_BREP('',#900);\nENDSEC;\nEND-ISO-10303-21;\n"
    }

    /// A cube is measured, and nothing is said of it before; with one face's loop wound the
    /// other way (as some writers wind them, and many readers put right in silence), the face is
    /// the rest of its plane, the file holds no solid, and that is said before any face is
    /// measured: the harness reads it even if the measure never ends.
    #[test]
    fn a_loop_wound_against_its_face_is_no_solid() {
        let (out, said) = answer(&a_cube(true), 1);
        assert!(said.is_empty(), "{:?}", said);
        assert!(
            (number(&out, "\"volume\": ") - 1.0).abs() < 1e-12,
            "{}",
            out
        );
        assert!((number(&out, "\"area\": ") - 6.0).abs() < 1e-12, "{}", out);
        for threads in [1, 3] {
            let (out, said) = answer(&a_cube(false), threads);
            let why = format!("face #122: {}", UNBOUNDED);
            assert_eq!(said, [why.clone()]);
            assert_eq!(out, format!("{{\"invalid\": \"{}\"}}", why));
        }
    }

    /// A quarter of a torus's tube, capped, its loop along the seam run against the standard (as
    /// Open CASCADE writes some): read by the standard the face is the other three quarters, and
    /// `by_seam` holds the quarter.
    #[test]
    fn a_seam_loop_run_backwards() {
        let s = format!(
            "ISO-10303-21;\nHEADER;\nENDSEC;\nDATA;\n{}\nENDSEC;\nEND-ISO-10303-21;\n",
            [
                "#1=(LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.));",
                "#2=CARTESIAN_POINT('',(0.,0.,0.));",
                "#3=DIRECTION('',(0.,0.,1.));",
                "#4=DIRECTION('',(1.,0.,0.));",
                "#5=AXIS2_PLACEMENT_3D('',#2,#3,#4);",
                "#6=TOROIDAL_SURFACE('',#5,10.,3.);",
                "#10=CARTESIAN_POINT('',(10.,0.,3.));",
                "#11=CARTESIAN_POINT('',(0.,10.,3.));",
                "#12=VERTEX_POINT('',#10);",
                "#13=VERTEX_POINT('',#11);",
                "#20=CARTESIAN_POINT('',(0.,0.,3.));",
                "#21=AXIS2_PLACEMENT_3D('',#20,#3,#4);",
                "#22=CIRCLE('',#21,10.);",
                "#23=EDGE_CURVE('',#12,#13,#22,.T.);",
                "#30=CARTESIAN_POINT('',(10.,0.,0.));",
                "#31=DIRECTION('',(0.,1.,0.));",
                "#32=AXIS2_PLACEMENT_3D('',#30,#31,#3);",
                "#33=CIRCLE('',#32,3.);",
                "#34=EDGE_CURVE('',#12,#12,#33,.T.);",
                "#40=CARTESIAN_POINT('',(0.,10.,0.));",
                "#41=DIRECTION('',(-1.,0.,0.));",
                "#42=AXIS2_PLACEMENT_3D('',#40,#41,#3);",
                "#43=CIRCLE('',#42,3.);",
                "#44=EDGE_CURVE('',#13,#13,#43,.T.);",
                "#50=ORIENTED_EDGE('',*,*,#23,.T.);",
                "#51=ORIENTED_EDGE('',*,*,#44,.F.);",
                "#52=ORIENTED_EDGE('',*,*,#23,.F.);",
                "#53=ORIENTED_EDGE('',*,*,#34,.T.);",
                "#54=EDGE_LOOP('',(#50,#51,#52,#53));",
                "#55=FACE_BOUND('',#54,.F.);",
                "#56=ADVANCED_FACE('',(#55),#6,.T.);",
                "#60=AXIS2_PLACEMENT_3D('',#30,#31,#3);",
                "#61=PLANE('',#60);",
                "#62=ORIENTED_EDGE('',*,*,#34,.F.);",
                "#63=EDGE_LOOP('',(#62));",
                "#64=FACE_BOUND('',#63,.T.);",
                "#65=ADVANCED_FACE('',(#64),#61,.F.);",
                "#70=AXIS2_PLACEMENT_3D('',#40,#41,#3);",
                "#71=PLANE('',#70);",
                "#72=ORIENTED_EDGE('',*,*,#44,.T.);",
                "#73=EDGE_LOOP('',(#72));",
                "#74=FACE_BOUND('',#73,.T.);",
                "#75=ADVANCED_FACE('',(#74),#71,.T.);",
                "#80=CLOSED_SHELL('',(#56,#65,#75));",
                "#81=MANIFOLD_SOLID_BREP('',#80);",
            ]
            .join("\n")
        );
        let (out, _) = answer(&s, 1);
        let seamed = &out[out.find("\"by_seam\"").unwrap()..];
        let (volume, area) = (45.0 * PI * PI, 30.0 * PI * PI + 18.0 * PI);
        assert!(
            (number(seamed, "\"volume\": ") / volume - 1.0).abs() < 1e-12,
            "{}",
            out
        );
        assert!(
            (number(seamed, "\"area\": ") / area - 1.0).abs() < 1e-12,
            "{}",
            out
        );
    }

    /// A ball of radius 5 less the cap beyond x + z = 5, as AnvilCAD writes it: the upper
    /// hemisphere's one loop runs round the equator and then the cut's circle, which touches the
    /// equator at (5, 0, 0) and runs through the pole. Its inner integral starts at that pole:
    /// from the other, a whole meridian weighs the noise in the angle's derivative by the pole.
    #[test]
    fn a_cut_through_a_pole_whose_loop_meets_itself() {
        let s = format!(
            "ISO-10303-21;\nHEADER;\nENDSEC;\nDATA;\n{}\nENDSEC;\nEND-ISO-10303-21;\n",
            [
                "#10=(LENGTH_UNIT()NAMED_UNIT(*)SI_UNIT(.MILLI.,.METRE.));",
                "#15=CARTESIAN_POINT('',(0.,0.,0.));",
                "#16=DIRECTION('',(0.,0.,1.));",
                "#17=DIRECTION('',(1.,0.,0.));",
                "#18=AXIS2_PLACEMENT_3D('',#15,#16,#17);",
                "#19=CARTESIAN_POINT('',(5.,0.,0.));",
                "#20=VERTEX_POINT('',#19);",
                "#21=CARTESIAN_POINT('',(0.,0.,0.));",
                "#22=DIRECTION('',(0.,0.,-1.));",
                "#23=DIRECTION('',(1.,0.,0.));",
                "#24=AXIS2_PLACEMENT_3D('',#21,#22,#23);",
                "#25=CIRCLE('',#24,5.);",
                "#26=EDGE_CURVE('',#20,#20,#25,.T.);",
                "#27=CARTESIAN_POINT('',(2.5,0.,2.5));",
                "#28=DIRECTION('',(0.7071067812,0.,0.7071067812));",
                "#29=DIRECTION('',(0.,1.,0.));",
                "#30=AXIS2_PLACEMENT_3D('',#27,#28,#29);",
                "#31=CIRCLE('',#30,3.5355339059);",
                "#32=EDGE_CURVE('',#20,#20,#31,.T.);",
                "#33=CARTESIAN_POINT('',(0.,0.,0.));",
                "#34=DIRECTION('',(0.,0.,-1.));",
                "#35=DIRECTION('',(1.,0.,0.));",
                "#36=AXIS2_PLACEMENT_3D('',#33,#34,#35);",
                "#37=SPHERICAL_SURFACE('',#36,5.);",
                "#38=ORIENTED_EDGE('',*,*,#26,.T.);",
                "#39=EDGE_LOOP('',(#38));",
                "#40=FACE_OUTER_BOUND('',#39,.T.);",
                "#41=ADVANCED_FACE('',(#40),#37,.T.);",
                "#42=CARTESIAN_POINT('',(0.,0.,0.));",
                "#43=DIRECTION('',(0.,0.,1.));",
                "#44=DIRECTION('',(1.,0.,0.));",
                "#45=AXIS2_PLACEMENT_3D('',#42,#43,#44);",
                "#46=SPHERICAL_SURFACE('',#45,5.);",
                "#47=ORIENTED_EDGE('',*,*,#26,.F.);",
                "#48=ORIENTED_EDGE('',*,*,#32,.F.);",
                "#49=EDGE_LOOP('',(#47,#48));",
                "#50=FACE_OUTER_BOUND('',#49,.T.);",
                "#51=ADVANCED_FACE('',(#50),#46,.T.);",
                "#52=CARTESIAN_POINT('',(2.5,0.,2.5));",
                "#53=DIRECTION('',(0.7071067812,0.,0.7071067812));",
                "#54=DIRECTION('',(0.,1.,0.));",
                "#55=AXIS2_PLACEMENT_3D('',#52,#53,#54);",
                "#56=PLANE('',#55);",
                "#57=ORIENTED_EDGE('',*,*,#32,.T.);",
                "#58=EDGE_LOOP('',(#57));",
                "#59=FACE_OUTER_BOUND('',#58,.T.);",
                "#60=ADVANCED_FACE('',(#59),#56,.T.);",
                "#61=CLOSED_SHELL('',(#41,#51,#60));",
                "#62=MANIFOLD_SOLID_BREP('Body1',#61);",
            ]
            .join("\n")
        );
        let (out, _) = answer(&s, 1);
        let (r, h) = (5.0f64, 5.0 * (1.0 - 0.5f64.sqrt()));
        let volume = 4.0 * PI * r.powi(3) / 3.0 - PI * h * h * (3.0 * r - h) / 3.0;
        let area = 4.0 * PI * r * r - 2.0 * PI * r * h + PI * r * r / 2.0;
        assert!(
            (number(&out, "\"volume\": ") / volume - 1.0).abs() < 1e-9,
            "{}",
            out
        );
        assert!(
            (number(&out, "\"area\": ") / area - 1.0).abs() < 1e-9,
            "{}",
            out
        );
    }
}
