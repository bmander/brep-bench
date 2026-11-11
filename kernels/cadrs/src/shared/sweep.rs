//! What the cadrs implementations of the sweep cases share: a solid sweep of a sketch along a
//! path, the path's pieces (sketch lines and arcs, Bézier curves in space), and a helix as
//! cadrs's Part Studio draws one.
//!
//! cadrs's kernel has no helical curve of its own: its Part Studio's Helix feature hands the
//! kernel the helix as rational Bézier pieces, sixteen a turn, each a rational quadratic lying on
//! the helix's cylinder (its arc of the circle, the height rising with it), and the kernel's
//! sweep recognises a path that follows a helix and sweeps the profile along it by the helix's
//! exact screw motion. [`helix`] draws it the same way.
use crate::bench::{P3, V3};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel, Helix, Kernel, PathCurve, Profile, SweepSpec};
use std::f64::consts::TAU;

/// The solid `profile`'s regions sweep along `path` (cadrs's Sweep, Solid, no profile control:
/// the profile keeps its angle to the path, corners mitred).
pub fn sweep(k: &mut DefaultKernel, profile: &Profile, path: Vec<PathCurve>) -> Result<BodyId, Refusal> {
    Ok(k.sweep_with(profile, &SweepSpec::solid(path))?.first()?)
}

/// [`sweep`], round the path's sharp corners by turning the profile about each corner (not
/// mitred).
pub fn sweep_round_corners(k: &mut DefaultKernel, profile: &Profile, path: Vec<PathCurve>) -> Result<BodyId, Refusal> {
    Ok(k.sweep_with(profile, &SweepSpec::solid(path).round_corners())?.first()?)
}

/// The conical helix about the axis through `origin` along +z, its radius going from `r0` to
/// `r1` as it rises `height`, starting at angle `start` (radians, from +x) and turning
/// right-handed (from +x toward +y) `turns` turns, radius, angle and height each in proportion:
/// as cadrs's Helix feature draws one on a cone, sixteen quintic Bézier pieces a turn, each
/// matching the helix's points and first and second derivatives at its ends.
pub fn conical_helix(origin: P3, r0: f64, r1: f64, height: f64, start: f64, turns: f64) -> Vec<PathCurve> {
    // (Quintic pieces even where the radii are equal: the kernel's own path draws a helix of
    // one radius exactly, as rational quadratics, and a case that asks for this drawing of it
    // means these.)
    let helix = Helix::conical(Axis::new(origin, V3::z()), r0, r1, height, start, turns);
    let n = ((turns.abs() * 16.0).ceil() as usize).max(1);
    (0..n)
        .map(|k| {
            let (s0, s1) = (k as f64 / n as f64, (k + 1) as f64 / n as f64);
            let h = s1 - s0;
            let (p0, d0, a0) = helix.at2(s0);
            let (p5, d5, a5) = helix.at2(s1);
            PathCurve::bezier(&[
                p0,
                p0 + d0 * (h / 5.0),
                p0 + d0 * (2.0 * h / 5.0) + a0 * (h * h / 20.0),
                p5 - d5 * (2.0 * h / 5.0) + a5 * (h * h / 20.0),
                p5 - d5 * (h / 5.0),
                p5,
            ])
        })
        .collect()
}

/// The helix about the axis through `origin` along +z of `radius`, rising `pitch` a turn,
/// starting at angle `start` (radians, from +x) and turning right-handed (from +x toward +y)
/// `turns` turns: as cadrs's Helix feature draws it, sixteen rational quadratic Bézier pieces
/// a turn, each the arc of the circle on its cylinder (its middle pole at the corner of the
/// tangents, weighted by the cosine of half its turn), the height rising with it.
pub fn helix(origin: P3, radius: f64, pitch: f64, start: f64, turns: f64) -> Vec<PathCurve> {
    Helix::new(Axis::new(origin, V3::z()), radius, pitch, start, turns).path()
}

/// The point at angle `t` (radians, from +x) of the helix about the z axis through `origin` of
/// `radius` rising `pitch` a turn from angle `start`, and its tangent there.
pub fn helix_at(origin: P3, radius: f64, pitch: f64, start: f64, t: f64) -> (P3, V3) {
    let z = pitch * (t - start) / TAU;
    (
        origin + V3::new(radius * t.cos(), radius * t.sin(), z),
        V3::new(-radius * t.sin(), radius * t.cos(), pitch / TAU),
    )
}

/// The solid `profile`'s regions sweep along `path`, twisting by `twist` (radians,
/// right-handed about the path) and scaling by `scale` (its size at the end over the start)
/// about the path's point, both in proportion to the length travelled (cadrs's
/// `Kernel::sweep_with_law`).
pub fn sweep_law(
    k: &mut DefaultKernel,
    profile: &Profile,
    path: Vec<PathCurve>,
    twist: f64,
    scale: f64,
) -> Result<BodyId, Refusal> {
    let law = cadrs_kernel::SweepLaw { twist, scale };
    Ok(k.sweep_with_law(profile, &SweepSpec::solid(path), &law)?.first()?)
}
