//! Helpers shared by the swept cases.
//!
//! springrep sweeps planar profiles (along a line, about an axis, along a screw or a curve), and
//! solids (`sweep::solid`), the latter exactly only under a slide, a turn or a screw. The cases that
//! build a swept solid without it, where it declines, build it from those profile sweeps where
//! its shape follows by elementary geometry, each case saying why in its comments. Three facts
//! carry most of them:
//!
//! - **A slide.** Under a slide along `v`, the line along `v` through a point of the swept solid
//!   meets the tool (a convex one) in a chord that moves along itself. Any flat sections of the tool
//!   that every such chord crosses, carried along `v` ([`slid`], an oblique prism), with the tool at
//!   both ends, make the whole sweep.
//! - **A turn.** Under a turn about an axis a point of the tool moves on a circle about it. Where
//!   every such circle meets the tool in arcs that each cross one half-plane through the axis, the
//!   swept solid is the tool at both ends and the tool's section by that half-plane turned through
//!   the angle ([`swing`]).
//! - **A screw.** A tool that is an upright prism of thickness `h`, screwed about the upright axis,
//!   stays at a height through the turn that raises it `h`: where it passes wholly, what it covers
//!   at each height is its level section turned through that angle, turned on with the height — a
//!   twisted extrusion ([`turned_bar_section`], [`screwed_bar`]). Where it starts or stops within
//!   the work, its level faces each sweep a twisted extrusion too ([`twisted`]), and a point of the
//!   swept solid lies in the tool where it starts or stops, under one of those, or on a circle
//!   about the axis that the screw carries through the tool in by one side and out by another,
//!   along an arc crossing a half-plane through the axis: the tool's section there, screwed
//!   ([`screwed_prism`]).
//!
//! The cases that call springrep's sweep of a solid (`sweep::solid`) take their motions from the
//! helpers at the end of this file ([`screwing`], [`turning`], [`carried`], [`tumble`]).
#![allow(dead_code)]

use std::f64::consts::PI;

use springrep::boolean::{intersection, union_all};
use springrep::edit::{Affine, affine};
use springrep::geom::{BSpline, Curve};
use springrep::math::{Frame, Point3, Transform, Vec2, Vec3};
use springrep::primitive::{cylinder, torus};
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::{Helix, Motion, extrude, helix, revolve, solid};
use springrep::topo::Brep;

use crate::shared::e;

/// Degrees in radians.
pub fn deg(a: f64) -> f64 {
    a * PI / 180.0
}

pub fn v2(x: f64, y: f64) -> Vec2 {
    Vec2::new(x, y)
}

pub fn v3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

/// `b` moved rigidly by `t`.
pub fn moved(b: &Brep, t: &Transform) -> Brep {
    let mut out = b.clone();
    out.transform(t);
    out
}

/// `b` turned by `angle` about the z axis, right-handed.
pub fn turned(b: &Brep, angle: f64) -> Brep {
    moved(b, &Transform::rotation(Point3::ZERO, Vec3::Z, angle))
}

/// `b` moved along `d`.
pub fn shifted(b: &Brep, d: Vec3) -> Brep {
    moved(b, &Transform::translation(d))
}

/// The union of `parts`.
pub fn unite(parts: &[&Brep]) -> Result<Brep, String> {
    union_all(parts).map_err(e)
}

/// The sketch frame of the half-plane through the z axis at polar angle `angle`: a point `(ρ, z)`
/// of the sketch is `ρ` out from the axis toward `angle` and at height `z`.
pub fn axial(angle: f64) -> Frame {
    let (s, c) = angle.sin_cos();
    Frame::new(Point3::ZERO, v3(s, -c, 0.0), v3(c, s, 0.0)).expect("a half-plane's frame")
}

/// The level frame at height `z`, its x axis at polar angle `angle`.
pub fn level(z: f64, angle: f64) -> Frame {
    let (s, c) = angle.sin_cos();
    Frame::new(v3(0.0, 0.0, z), Vec3::Z, v3(c, s, 0.0)).expect("a level frame")
}

/// `profile`, drawn in the half-plane through the z axis at polar angle `from` (sketch x out from
/// the axis, y up), turned about the z axis through `by`, right-handed.
pub fn swing(profile: &Profile, from: f64, by: f64, feature: &str) -> Result<Brep, String> {
    revolve(profile, &axial(from), Point3::ZERO, Vec3::Z, by, feature).map_err(e)
}

/// The axis-aligned rectangle from `(u0, v0)` to `(u1, v1)` in sketch coordinates.
pub fn rect(u0: f64, v0: f64, u1: f64, v1: f64) -> Profile {
    Profile::rect(v2((u0 + u1) / 2.0, (v0 + v1) / 2.0), u1 - u0, v1 - v0)
}

/// The polygon through `points`, every edge named `name`.
pub fn polygon(points: &[(f64, f64)], name: &str) -> Profile {
    let p: Vec<Vec2> = points.iter().map(|&(x, y)| v2(x, y)).collect();
    Profile::polygon(&p, &[name])
}

/// The curved block of the ring-sector cases: everything between `r0` and `r1` from the z axis,
/// between heights `z0` and `z1`, and between polar angles `a0` and `a1`: a rectangle turned.
pub fn ring_sector(r0: f64, r1: f64, z0: f64, z1: f64, a0: f64, a1: f64) -> Result<Brep, String> {
    swing(&rect(r0, z0, r1, z1), a0, a1 - a0, "block")
}

/// The block of most ring-sector cases: radii 8 to 12, heights −1 to 1, angles 0° to 60°.
pub fn sector_block() -> Result<Brep, String> {
    ring_sector(8.0, 12.0, -1.0, 1.0, 0.0, deg(60.0))
}

/// The rod of the screw cases: the cylinder of radius 5 about the z axis from z = 0 to z = 20.
pub fn rod() -> Result<Brep, String> {
    cylinder(&Frame::WORLD, 5.0, 20.0, "rod").map_err(e)
}

/// `profile`, drawn on `frame`, carried along `v` (which must not lie in its plane): the oblique
/// prism a flat section of a tool sweeps as the tool slides along `v`. It is the right prism of the
/// profile, extruded along the frame's normal, sheared so its far cap lands at `v`; the shear keeps
/// the profile's plane where it is, and maps every surface exactly.
pub fn slid(profile: &Profile, frame: &Frame, v: Vec3, feature: &str) -> Result<Brep, String> {
    let n = frame.z;
    let d = v.dot(n);
    if d.abs() < 1e-9 * v.norm() {
        return Err("a section slid along its own plane sweeps no volume".into());
    }
    let prism = extrude(profile, frame, d, feature).map_err(e)?;
    if (v - n * d).norm() < 1e-12 * v.norm() {
        return Ok(prism);
    }
    // x ↦ x + ((x − o)·n / d)(v − d n): the plane through o square to n stays, o + d n goes to o + v
    let w = (v - n * d) * (1.0 / d);
    let (wa, na) = (w.to_array(), n.to_array());
    let mut m = [[0.0; 3]; 3];
    for (i, row) in m.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = if i == j { 1.0 } else { 0.0 } + wa[i] * na[j];
        }
    }
    let t = w * -(frame.origin.dot(n));
    affine(&prism, &Affine { m, t }).map_err(e)
}

/// A screw about the z axis rising `lead` for every whole turn.
pub fn screw(lead: f64) -> Helix {
    Helix {
        origin: Point3::ZERO,
        axis: Vec3::Z,
        rise: lead / (2.0 * PI),
        sign: 1.0,
    }
}

/// The twisted extrusion of a level face: `profile`, drawn in the level plane at height `z` with
/// its x axis at polar angle `from`, screwed about the z axis with `lead` through `by` radians —
/// the solid a level face of a tool sweeps under that screw, one copy of the face at each height.
pub fn twisted(
    profile: &Profile,
    z: f64,
    from: f64,
    lead: f64,
    by: f64,
    feature: &str,
) -> Result<Brep, String> {
    helix(profile, &level(z, from), &screw(lead), by, feature).map_err(e)
}

/// `profile`, drawn in the half-plane at polar angle `from` (sketch x out from the axis, y up),
/// screwed about the z axis with `lead` through `by` radians.
pub fn screwed_section(
    profile: &Profile,
    from: f64,
    lead: f64,
    by: f64,
    feature: &str,
) -> Result<Brep, String> {
    helix(profile, &axial(from), &screw(lead), by, feature).map_err(e)
}

/// The pieces of the solid an upright prism fills as it is screwed about the z axis: the prism is
/// `base`, drawn in the level plane (its x axis along +x), standing from height `z0` to `z1`; the
/// screw rises `lead` a turn and turns from `t0` to `t1` radians, the prism rising `lead·t/2π` at
/// turn `t`.
///
/// Each of `sections` is a half-plane through the axis, `(angle, ρ0, ρ1)`, where the prism holds
/// everything from `ρ0` to `ρ1` out between its two heights; together they must cross every arc in
/// which a circle about the axis meets the base. The swept solid is then the prism where it starts
/// and where it stops (left out when `ends` is false, for a tool clear of the work there), its top
/// and bottom faces' twisted extrusions, and those sections screwed (see the module's comment).
#[allow(clippy::too_many_arguments)]
pub fn screwed_prism_parts(
    base: &Profile,
    z0: f64,
    z1: f64,
    lead: f64,
    t0: f64,
    t1: f64,
    sections: &[(f64, f64, f64)],
    ends: bool,
) -> Result<Vec<Brep>, String> {
    let rise = |t: f64| lead * t / (2.0 * PI);
    let by = t1 - t0;
    let mut parts = Vec::new();
    for &(angle, r0, r1) in sections {
        let section = rect(r0, z0 + rise(t0), r1, z1 + rise(t0));
        parts.push(screwed_section(&section, angle + t0, lead, by, "section")?);
    }
    for z in [z0, z1] {
        parts.push(twisted(base, z + rise(t0), t0, lead, by, "face")?);
    }
    if ends {
        for t in [t0, t1] {
            let prism = extrude(base, &level(z0 + rise(t), t), z1 - z0, "tool").map_err(e)?;
            parts.push(prism);
        }
    }
    Ok(parts)
}

/// The line segment from `a` to `b`.
pub fn line(a: (f64, f64), b: (f64, f64), name: &str) -> NamedSegment {
    NamedSegment::line(v2(a.0, a.1), v2(b.0, b.1), name)
}

/// The arc about `c` of `radius` from angle `start` through `sweep` (radians).
pub fn arc(c: (f64, f64), radius: f64, start: f64, sweep: f64, name: &str) -> NamedSegment {
    NamedSegment::arc(v2(c.0, c.1), radius, start, sweep, name)
}

/// The box from `lo` to `hi`.
pub fn boxed(lo: [f64; 3], hi: [f64; 3], feature: &str) -> Result<Brep, String> {
    let size = v3(hi[0] - lo[0], hi[1] - lo[1], hi[2] - lo[2]);
    springrep::primitive::block(&crate::shared::at(lo), size, feature).map_err(e)
}

/// A tool of revolution about the upright line through `(cx, cy)`: `meridian` drawn with sketch x
/// out from that line (toward +x) and y up, from the line outward, turned a whole turn about it.
pub fn upright(meridian: &Profile, cx: f64, cy: f64, feature: &str) -> Result<Brep, String> {
    let foot = v3(cx, cy, 0.0);
    let sketch = Frame::new(foot, -Vec3::Y, Vec3::X).map_err(e)?;
    revolve(meridian, &sketch, foot, Vec3::Z, 2.0 * PI, feature).map_err(e)
}

/// The profile of `segments` end to end.
pub fn outline(segments: Vec<NamedSegment>) -> Profile {
    Profile::outline(segments)
}

/// The box about `center` with edges along the orthonormal `axes`, reaching `half[i]` along each.
pub fn cuboid(
    center: Vec3,
    axes: [Vec3; 3],
    half: [f64; 3],
    feature: &str,
) -> Result<Brep, String> {
    let base = Frame::new(center - axes[2] * half[2], axes[2], axes[0]).map_err(e)?;
    let face = Profile::rect(Vec2::ZERO, 2.0 * half[0], 2.0 * half[1]);
    extrude(&face, &base, 2.0 * half[2], feature).map_err(e)
}

/// The stadium of the points within `r` of the segment from `(a, 0)` to `(b, 0)` (a < b), in
/// sketch coordinates.
pub fn stadium(a: f64, b: f64, r: f64, name: &str) -> Profile {
    let half = PI / 2.0;
    outline(vec![
        line((a, -r), (b, -r), name),
        arc((b, 0.0), r, -half, PI, name),
        line((b, r), (a, r), name),
        arc((a, 0.0), r, half, PI, name),
    ])
}

/// The ring (torus) about the line through `center` along `axis`, its tube's centers `major`
/// from it and its tube of radius `minor`, with its hole filled, cut in half by the plane through
/// its axis square to `across` and kept on the side `across` points to: `minor` round the rim of
/// a disc of radius `major`, half of it. A ring slid square to its axis farther than its hole is
/// wide ends in these.
pub fn half_filled_ring(
    center: Vec3,
    axis: Vec3,
    across: Vec3,
    major: f64,
    minor: f64,
    feature: &str,
) -> Result<Brep, String> {
    let half = PI / 2.0;
    // sketch x out from the axis (along `out`, square to `across`), y along it
    let out = axis.cross(across);
    let sketch = Frame::new(center, out.cross(axis), out).map_err(e)?;
    let meridian = outline(vec![
        line((0.0, -minor), (major, -minor), feature),
        arc((major, 0.0), minor, -half, PI, feature),
        line((major, minor), (0.0, minor), feature),
        line((0.0, minor), (0.0, -minor), feature),
    ]);
    // the half turn about the axis that carries `out` through `across`
    let dir = if out.cross(across).dot(axis) > 0.0 {
        axis
    } else {
        -axis
    };
    revolve(&meridian, &sketch, center, dir, PI, feature).map_err(e)
}

/// The solid a ring fills as it slides square to its own axis, by `d` along the unit `along`,
/// farther than its hole is wide (`d` ≥ 2·`major`): the points within `minor` of the stadium its
/// tube's center circle sweeps, a disc of radius `major` slid along. That is the stadium section
/// square to the slide (the tube's two sides and the band between them) carried the length of the
/// slide, and half the ring with its hole filled at each end.
pub fn slid_ring(
    center: Vec3,
    axis: Vec3,
    along: Vec3,
    major: f64,
    minor: f64,
    d: f64,
    feature: &str,
) -> Result<Brep, String> {
    let across = along.cross(axis);
    // the section square to the slide through the center: sketch x across, y along the axis
    let sketch = Frame::new(center, along, across).map_err(e)?;
    let band = slid(
        &stadium(-major, major, minor, feature),
        &sketch,
        along * d,
        feature,
    )?;
    let back = half_filled_ring(center, axis, -along, major, minor, feature)?;
    let front = half_filled_ring(center + along * d, axis, along, major, minor, feature)?;
    unite(&[&back, &band, &front])
}

/// What a bar pointing at the z axis covers in a level plane as it turns about the axis through
/// `delta` radians, within `reach` of the axis: the bar's section there, everything with
/// x ≥ `tip` and |y| ≤ `half` (no bound on y for `None`) out to `reach`, turned from 0 to `delta`.
/// Every circle about the axis meets the section in one arc from its trailing side to its leading
/// one, so what it covers is bounded by its trailing side where it starts, its leading side where
/// it stops, and the arcs its nearest point (the tip's middle, at `tip`) and its outermost corners
/// (at `reach`) trace between. `delta` must leave a gap between the two sides.
pub fn turned_bar_section(tip: f64, half: Option<f64>, reach: f64, delta: f64) -> Profile {
    let turn = |(x, y): (f64, f64)| {
        let (s, c) = delta.sin_cos();
        (x * c - y * s, x * s + y * c)
    };
    let mut segs = Vec::new();
    match half {
        None => {
            let h = (reach * reach - tip * tip).sqrt();
            let corner = h.atan2(tip);
            segs.push(line((tip, 0.0), (tip, -h), "trailing"));
            segs.push(arc(
                (0.0, 0.0),
                reach,
                -corner,
                delta + 2.0 * corner,
                "outer",
            ));
            segs.push(line(turn((tip, h)), turn((tip, 0.0)), "leading"));
        }
        Some(w) => {
            let x = (reach * reach - w * w).sqrt();
            let corner = w.atan2(x);
            segs.push(line((tip, 0.0), (tip, -w), "trailing"));
            segs.push(line((tip, -w), (x, -w), "trailing"));
            segs.push(arc(
                (0.0, 0.0),
                reach,
                -corner,
                delta + 2.0 * corner,
                "outer",
            ));
            segs.push(line(turn((x, w)), turn((tip, w)), "leading"));
            segs.push(line(turn((tip, w)), turn((tip, 0.0)), "leading"));
        }
    }
    segs.push(arc((0.0, 0.0), tip, delta, -delta, "root"));
    outline(segs)
}

/// The groove a bar pointing at the z axis cuts as it is screwed about the axis, rising `lead` a
/// turn, from turn `t0` to `t1` (radians), the bar `thick` thick along the axis about height 0 and
/// its section within reach of the work `section` (as [`turned_bar_section`] makes it, turned
/// through `delta`, the turn in which the screw raises it its own thickness).
///
/// At a height where the bar passes wholly, the screw carries it there for the turn `delta` that
/// raises it its thickness, so what it covers there is `section`, turned with the height: a
/// twisted extrusion of it, from where the bar's top passes as it starts to where its bottom passes
/// as it stops.
pub fn screwed_bar(
    section: &Profile,
    thick: f64,
    lead: f64,
    t0: f64,
    t1: f64,
    delta: f64,
) -> Result<Brep, String> {
    let rise = |t: f64| lead * t / (2.0 * PI);
    twisted(
        section,
        thick / 2.0 + rise(t0),
        t0,
        lead,
        t1 - t0 - delta,
        "groove",
    )
}

/// The solid of [`screwed_prism_parts`], united.
#[allow(clippy::too_many_arguments)]
pub fn screwed_prism(
    base: &Profile,
    z0: f64,
    z1: f64,
    lead: f64,
    t0: f64,
    t1: f64,
    sections: &[(f64, f64, f64)],
    ends: bool,
) -> Result<Brep, String> {
    let parts = screwed_prism_parts(base, z0, z1, lead, t0, t1, sections, ends)?;
    let refs: Vec<&Brep> = parts.iter().collect();
    unite(&refs)
}

/// A tool of revolution about the line through `base` along `axis`: `meridian` drawn with sketch x
/// along `across` (square to `axis`) and y along `axis`, turned a whole turn about that line.
pub fn revolved(
    meridian: &Profile,
    base: Vec3,
    axis: Vec3,
    across: Vec3,
    feature: &str,
) -> Result<Brep, String> {
    let sketch = Frame::new(base, across.cross(axis), across).map_err(e)?;
    revolve(meridian, &sketch, base, axis, 2.0 * PI, feature).map_err(e)
}

/// The screw about the z axis rising `lead` a turn, from turn `from` to turn `to` (radians).
pub fn screwing(lead: f64, from: f64, to: f64) -> Motion {
    Motion::Screw {
        screw: screw(lead),
        from,
        to,
    }
}

/// The turn about the z axis from `from` to `to` (radians).
pub fn turning(from: f64, to: f64) -> Motion {
    Motion::Turn {
        origin: Point3::ZERO,
        axis: Vec3::Z,
        from,
        to,
    }
}

/// The motion of the carried cases: along the cubic Bézier curve through (0, 0, 0), (10, 0, 0),
/// (20, 20, 0) and (30, 20, 0), the tool's x along the curve and its z up.
pub fn carried() -> Result<Motion, String> {
    let ctrl = vec![
        v3(0.0, 0.0, 0.0),
        v3(10.0, 0.0, 0.0),
        v3(20.0, 20.0, 0.0),
        v3(30.0, 20.0, 0.0),
    ];
    let knots = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let path = Curve::BSpline(BSpline::new(3, knots, ctrl).map_err(e)?);
    Ok(Motion::Along {
        path,
        range: (0.0, 1.0),
        up: Vec3::Z,
    })
}

/// The part of `tool` within `radius` of the z axis. A screw or a turn about the axis keeps each
/// point's distance from it, so only that part of a tool can reach anything within that radius.
pub fn within(tool: &Brep, radius: f64) -> Result<Brep, String> {
    let reach = cylinder(
        &crate::shared::at([0.0, 0.0, -100.0]),
        radius,
        200.0,
        "reach",
    );
    intersection(tool, &reach.map_err(e)?).map_err(e)
}

/// The tumble of the tumbling cases as the roll t runs from `from` to `to` (radians): turned by
/// `rate`·t about the upright line through (2, 0, 0), then by `turn`·t about the line through
/// (0, h cos ℓ, h sin ℓ) along (cos α, sin α cos ℓ, sin α sin ℓ), for `(ℓ, α, h)`.
pub fn tumble(rate: f64, turn: f64, (l, a, h): (f64, f64, f64), from: f64, to: f64) -> Motion {
    Motion::Then(
        Box::new(Motion::Turn {
            origin: v3(2.0, 0.0, 0.0),
            axis: Vec3::Z,
            from: rate * from,
            to: rate * to,
        }),
        Box::new(Motion::Turn {
            origin: v3(0.0, h * l.cos(), h * l.sin()),
            axis: v3(a.cos(), a.sin() * l.cos(), a.sin() * l.sin()),
            from: turn * from,
            to: turn * to,
        }),
    )
}

/// The first tumble: a quarter of the roll about the upright line, the roll back about the line
/// through (0, 0.5, 0.5) along x.
pub fn tumbling(from: f64, to: f64) -> Motion {
    tumble(0.25, -1.0, (deg(45.0), 0.0, 0.5f64.sqrt()), from, to)
}

/// What the torus about the upright line through `center`, of major radius 1 and tube radius
/// `minor`, passes through under `motion`.
pub fn tumbled(center: Vec3, minor: f64, motion: &Motion) -> Result<Brep, String> {
    let ring = torus(&Frame::WORLD.translated(center), 1.0, minor, "ring").map_err(e)?;
    solid(&ring, motion, "tumble").map_err(e)
}

/// One of the crossed pair of torus tumbles: turned back about the second line when `turn` is −1,
/// on when it is 1.
pub fn crossed(turn: f64) -> Result<Brep, String> {
    let motion = tumble(
        0.7,
        turn,
        (deg(45.0), deg(20.0), 0.7071),
        deg(-100.0),
        deg(100.0),
    );
    tumbled(v3(3.0, 0.0, 2.0), 0.22, &motion)
}
