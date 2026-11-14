//! What the cadrs implementations of the swept cases share: the rod and the ring sector several of
//! them groove, a slide, a swing or a screw of a solid (cadrs's sweep of a solid,
//! `Kernel::sweep_solid`), the solid a torus fills as it slides along the z axis, and the gear
//! ladder's blanks, pinion tooth and worm thread, cut under a pair's two turns.
use crate::bench::{arc, cylinder, extrude, p2, p3, torus, v3, P3, V3};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, Curve2, CylindricalBox, DefaultKernel, Kernel, Loop, Plane, Profile, RigidStep, SolidMotion};

/// The rod: the cylinder of radius 5 about the z axis from z = 0 to z = 20.
pub fn rod(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 5.0, 20.0)
}

/// The block of a ring sector about the z axis: everything between the radii `radii`, the
/// heights `heights` and the polar angles `angles` (degrees, from +x toward +y), cadrs's
/// cylindrical box (an annular sector extruded).
pub fn ring_sector(
    k: &mut DefaultKernel,
    radii: (f64, f64),
    heights: (f64, f64),
    angles: (f64, f64),
) -> Result<BodyId, Refusal> {
    let start = angles.0.rem_euclid(360.0);
    let spec = CylindricalBox {
        frame: Plane::new(p3(0.0, 0.0, heights.0), v3(1.0, 0.0, 0.0), v3(0.0, 0.0, 1.0)),
        length: heights.1 - heights.0,
        inner_radius: radii.0,
        outer_radius: radii.1,
        start_angle: start.to_radians(),
        end_angle: (start + angles.1 - angles.0).to_radians(),
    };
    Ok(k.cylindrical_box(&spec)?.first()?)
}

/// The z axis.
pub fn z_axis() -> Axis {
    Axis::new(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0))
}

/// Everything `tool` passes through as it slides without turning along `v` (cadrs's sweep of a
/// solid, one slide).
pub fn slid(k: &mut DefaultKernel, tool: BodyId, v: V3) -> Result<BodyId, Refusal> {
    Ok(k.sweep_solid(tool, &slide(v))?.first()?)
}

/// Everything `tool` (drawn where it is at a turn of 0) passes through as it turns rigidly about
/// `about`, right-handed, from `from` to `to` degrees (cadrs's sweep of a solid, one turn).
pub fn swung(k: &mut DefaultKernel, tool: BodyId, about: Axis, from: f64, to: f64) -> Result<BodyId, Refusal> {
    Ok(k.sweep_solid(tool, &swing(about, from, to))?.first()?)
}

/// Everything `tool` (drawn where it is at a turn of 0) passes through as it is screwed about
/// `about`: turned right-handed from `from` to `to` degrees while rising `lead` along the axis a
/// whole turn, rigidly (cadrs's sweep of a solid: a turn and a slide along its axis at once).
pub fn screwed(
    k: &mut DefaultKernel,
    tool: BodyId,
    about: Axis,
    lead: f64,
    from: f64,
    to: f64,
) -> Result<BodyId, Refusal> {
    Ok(k.sweep_solid(tool, &screw(about, lead, from, to))?.first()?)
}

/// Everything `tool` (drawn in the frame where `path` starts) passes through as a frame rides
/// `path` upright: its origin the path's point, its x axis the path's tangent, its z axis up
/// (cadrs's sweep of a solid along a path: exact for a prism in its plane, a torus, and an end
/// or bull-nose mill; else the region its places hold, meshed).
pub fn carried(k: &mut DefaultKernel, tool: BodyId, path: Vec<cadrs_kernel::PathCurve>) -> Result<BodyId, Refusal> {
    Ok(k.sweep_solid(tool, &carry(path))?.first()?)
}

/// The cubic Bézier curve in the plane z = 0 on these control points, as a path.
pub fn bezier_path(poles: [(f64, f64); 4]) -> Vec<cadrs_kernel::PathCurve> {
    vec![cadrs_kernel::PathCurve::bezier(&poles.map(|(x, y)| p3(x, y, 0.0)))]
}

/// `target` less everything `tool` passes through as it moves by `motion` (cadrs's cut by a swept
/// solid: the exact sweep cut from it, else the region inside it and in none of the tool's
/// places, meshed).
pub fn cut_swept(k: &mut DefaultKernel, target: BodyId, tool: BodyId, motion: &SolidMotion) -> Result<BodyId, Refusal> {
    Ok(k.cut_swept(target, tool, motion)?.first()?)
}

/// [`cut_swept`], refused where cadrs makes the cut only as a mesh (the region inside the target
/// and in none of the tool's places, contoured on a grid): a body of flat facets is not the
/// solid the case describes.
pub fn cut_swept_exactly(
    k: &mut DefaultKernel,
    target: BodyId,
    tool: BodyId,
    motion: &SolidMotion,
) -> Result<BodyId, Refusal> {
    let made = cut_swept(k, target, tool, motion)?;
    let body = k.get(made)?;
    let flat = body
        .faces()
        .iter()
        .filter(|f| matches!(f.surf, cadrs_kernel::native::Surface::Plane { .. }))
        .count();
    let curved = body.faces().len() - flat;
    if curved == 0 && flat > 1000 {
        return Err(Refusal::from(
            "cadrs cuts this by its moving tool only as a mesh of flat facets, not as the solid's own faces",
        ));
    }
    Ok(made)
}

/// The motion of [`slid`].
pub fn slide(v: V3) -> SolidMotion {
    SolidMotion::slide(v)
}

/// The motion of [`swung`].
pub fn swing(about: Axis, from: f64, to: f64) -> SolidMotion {
    SolidMotion::swing(about, from.to_radians(), to.to_radians())
}

/// The motion of [`screwed`].
pub fn screw(about: Axis, lead: f64, from: f64, to: f64) -> SolidMotion {
    SolidMotion::helical(about, lead, from.to_radians(), to.to_radians())
}

/// The motion of [`carried`].
pub fn carry(path: Vec<cadrs_kernel::PathCurve>) -> SolidMotion {
    SolidMotion::along(path, V3::z())
}

/// The part of `tool` within `radius` of the z axis (cadrs's boolean common with a cylinder about
/// it). A turn or screw about the z axis maps the cylinder of any radius about it onto itself, so
/// what the tool passes through within the rod (radius 5) is what this part of it passes through
/// there, for any radius past 5.
pub fn near_axis(k: &mut DefaultKernel, tool: BodyId, radius: f64) -> Result<BodyId, Refusal> {
    let bb = k.bounding_box(tool)?;
    let (z0, z1) = (bb.min.z - 1.0, bb.max.z + 1.0);
    let c = cylinder(k, p3(0.0, 0.0, z0), v3(0.0, 0.0, 1.0), radius, z1 - z0)?;
    crate::bench::common(k, tool, c)
}

/// What the torus about the axis through `center` parallel to the y axis, of radii `major` and
/// `minor`, fills as it slides without turning `rise` up the z axis: the torus where it starts
/// and where it ends, and between them the region it covers seen along the slide (its shadow on
/// the plane z = `center.z`, the stadium of half-width `minor` about the segment from
/// `center.x − major` to `center.x + major` along x) extruded `rise` up. Every point the torus
/// passes lies in one of them.
pub fn torus_slid(
    k: &mut DefaultKernel,
    center: P3,
    major: f64,
    minor: f64,
    rise: f64,
) -> Result<Vec<BodyId>, Refusal> {
    let start = torus(k, center, v3(0.0, 1.0, 0.0), major, minor)?;
    let end = torus(k, center + v3(0.0, 0.0, rise), v3(0.0, 1.0, 0.0), major, minor)?;
    let (x0, x1, y, r) = (center.x - major, center.x + major, center.y, minor);
    let stadium = Loop::new(vec![
        Curve2::line(p2(x0, y - r), p2(x1, y - r)),
        arc(p2(x1, y), r, -90.0, 180.0),
        Curve2::line(p2(x1, y + r), p2(x0, y + r)),
        arc(p2(x0, y), r, 90.0, 180.0),
    ]);
    let shadow = extrude(k, &Profile::of(Plane::xy(center.z), vec![stadium]), rise)?;
    Ok(vec![start, shadow, end])
}

/// A tumbling torus (`swept/tumbling-torus-*`): what the torus about `center` (axis along z, major
/// radius 1, tube radius `tube`) fills as, at each roll t from `rolls.0` to `rolls.1` degrees, it is
/// turned by `rate`·t about the upright line through (2, 0, 0), then by `sign`·t about the line
/// through q = (0, h cos ℓ, h sin ℓ) along u = (cos α, sin α cos ℓ, sin α sin ℓ), `(ℓ, α, h)` the
/// second axis's lean, tilt and offset (degrees, degrees, length). Cadrs's sweep of a solid, with
/// the rigid motion of its two turns, the second axis fixed in the world.
pub fn tumbling_torus(
    k: &mut DefaultKernel,
    center: P3,
    tube: f64,
    (lean, tilt, h): (f64, f64, f64),
    rate: f64,
    sign: f64,
    rolls: (f64, f64),
) -> Result<BodyId, Refusal> {
    let t = crate::bench::torus(k, center, v3(0.0, 0.0, 1.0), 1.0, tube)?;
    let (l, a) = (lean.to_radians(), tilt.to_radians());
    let q = p3(0.0, h * l.cos(), h * l.sin());
    let u = v3(a.cos(), a.sin() * l.cos(), a.sin() * l.sin());
    let (t0, t1) = (rolls.0.to_radians(), rolls.1.to_radians());
    let motion = SolidMotion::rigid(vec![
        RigidStep::turn(Axis::new(p3(2.0, 0.0, 0.0), v3(0.0, 0.0, 1.0)), rate * t0, rate * t1),
        RigidStep::turn(Axis::new(q, u), sign * t0, sign * t1),
    ]);
    Ok(k.sweep_solid(t, &motion)?.first()?)
}

/// The two operands of `swept/tumbling-torus-crossed-*`: the torus (center (3, 0, 2), tube 0.22)
/// rolled from −100° to 100° at 0.7 about the upright axis, the second axis leaning 45° and
/// tilted 20° at 0.7071, turned against the roll (A) and with it (B).
pub fn crossed_tori(k: &mut DefaultKernel) -> Result<(BodyId, BodyId), Refusal> {
    Ok((crossed_torus(k, -1.0)?, crossed_torus(k, 1.0)?))
}

/// One operand of `swept/tumbling-torus-crossed-*`: its second axis turned against the roll
/// (`sign` −1: A, the forward tumble) or with it (+1: B, the reverse).
pub fn crossed_torus(k: &mut DefaultKernel, sign: f64) -> Result<BodyId, Refusal> {
    tumbling_torus(k, p3(3.0, 0.0, 2.0), 0.22, (45.0, 20.0, 0.7071), 0.7, sign, (-100.0, 100.0))
}

/// The gear ladder's bevel pair (`swept/straight-bevel-gear-cut-by-its-pinion`,
/// `swept/hypoid-gear-cut-by-its-pinion`): g along the line where the pitch cones touch, d square
/// to it in the xz-plane, from the pinion into the gear.
fn bevel_frame() -> (V3, V3) {
    let s5 = 5f64.sqrt();
    (v3(2.0 / s5, 0.0, 1.0 / s5), v3(-1.0 / s5, 0.0, 2.0 / s5))
}

/// The bevel gear's blank: the quadrilateral of the points s·g + u·d (s from 15√5 − 10 to 15√5,
/// u from 6 inside the pitch cone to the face cone through O, 2 outside it at the outer end) on
/// the Front plane, revolved about the z axis.
pub fn bevel_blank(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let (g, d) = bevel_frame();
    let (ae, ai) = (15.0 * 5f64.sqrt(), 15.0 * 5f64.sqrt() - 10.0);
    let at = |s: f64, u: f64| {
        let p = g * s + d * u;
        [p.x, p.z]
    };
    let corners = [at(ae, -2.0), at(ae, 6.0), at(ai, 6.0), at(ai, -2.0 * ai / ae)];
    let quad = Profile::of(Plane::xz(0.0), vec![Loop::polygon(&corners)]);
    crate::bench::revolve(k, &quad, z_axis(), None)
}

/// The pinion's tooth: the pyramid with apex O over the trapezoid P + u·d + w·y (P = 15√5·g,
/// |u| ≤ 2.5, |w| ≤ π/2 − u·tan 20°), between the planes square to g at 15√5 − 12 and 15√5 + 2.
/// The trapezoid, scaled out to the outer plane, lofted to the apex; then the part beyond the
/// inner plane kept (common with a slab square to g).
pub fn bevel_tooth(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    use cadrs_kernel::{BodyKind, LoftEnd, LoftSection, LoftSpec};
    let (g, d) = bevel_frame();
    let ae = 15.0 * 5f64.sqrt();
    let (inner, outer) = (ae - 12.0, ae + 2.0);
    let scale = outer / ae;
    let tan20 = 20f64.to_radians().tan();
    let w = |u: f64| std::f64::consts::FRAC_PI_2 - u * tan20;
    // sketch x along d, sketch y along g × d = −y, on the outer plane
    let base = Plane::new(P3::origin() + g * outer, d, g);
    let trapezoid = [(2.5, w(2.5)), (-2.5, w(-2.5)), (-2.5, -w(-2.5)), (2.5, -w(2.5))]
        .map(|(u, w)| [u * scale, w * scale]);
    let spec = LoftSpec {
        body: BodyKind::Solid,
        sections: vec![
            LoftSection::Profile(Profile::of(base, vec![Loop::polygon(&trapezoid)])),
            LoftSection::Point(P3::origin()),
        ],
        start: LoftEnd::default(),
        end: LoftEnd::default(),
        source: 1,
    };
    let pyramid = k.loft_with(&spec)?.first()?;
    // the slab from the inner plane to past the outer one, wide enough to hold the tooth
    let slab_base = Plane::new(P3::origin() + g * inner, d, g);
    let slab = extrude(k, &Profile::of(slab_base, vec![Loop::rectangle(-10.0, -10.0, 10.0, 10.0)]), 15.0)?;
    crate::bench::common(k, pyramid, slab)
}

/// The blank less what `tooth` passes through as it turns about `pinion` by `roll` (degrees)
/// while the gear turns by `ratio` times it about the z axis (seen from the gear, by −ratio
/// times it): cadrs's cut by a moving tool (`Kernel::cut_swept`) under the two turns, applied in
/// order at each instant. The tooth space's faces are the sheets the tooth's contacts and
/// edges generate, each fitted by one B-spline.
pub fn bevel_cut(
    k: &mut DefaultKernel,
    tooth: BodyId,
    pinion: Axis,
    ratio: f64,
    roll: (f64, f64),
) -> Result<BodyId, Refusal> {
    let (t0, t1) = (roll.0.to_radians(), roll.1.to_radians());
    let motion = SolidMotion::Rigid {
        steps: vec![
            RigidStep::Turn { axis: pinion, from: t0, to: t1 },
            RigidStep::Turn { axis: z_axis(), from: -ratio * t0, to: -ratio * t1 },
        ],
    };
    let blank = bevel_blank(k)?;
    cut_swept_exactly(k, blank, tooth, &motion)
}

/// The gear ladder's cylindrical worm (`swept/helicon-face-gear-hobbed-by-its-worm`,
/// `swept/worm-wheel-hobbed-by-its-worm`): its thread on the line through `at` along the x axis,
/// from x = `from` to `to`, lead 2π, one start, right-handed. Built about the z axis: its axial
/// section, the trapezoid π wide at radius 10 from radius 7.5 to 12.5, its sides 20° from square to
/// the axis, on the Front plane, swept along the helix of radius 10 through (10, 0, ·) (cadrs's
/// sweep follows it by the helix's screw motion, the section kept in its plane through the axis) a
/// turn past each end, and cut to the planes z = `from` and `to`; then turned by 90° about the y
/// axis (z onto x, the half-plane x > 0 onto −z, a quarter turn behind +y about the new axis, so the
/// section is centred a quarter turn back, z ≡ −π/2) and moved to `at`.
pub fn worm_thread(k: &mut DefaultKernel, at: P3, from: f64, to: f64) -> Result<BodyId, Refusal> {
    use std::f64::consts::{FRAC_PI_2, TAU};
    let tan20 = 20f64.to_radians().tan();
    let w = |r: f64| FRAC_PI_2 - (r - 10.0) * tan20;
    let turns_before = (-from / TAU).ceil() + 1.0;
    let start = -TAU * turns_before;
    let mid = start - FRAC_PI_2;
    let section = [[7.5, mid - w(7.5)], [12.5, mid - w(12.5)], [12.5, mid + w(12.5)], [7.5, mid + w(7.5)]];
    let profile = Profile::of(Plane::xz(0.0), vec![Loop::polygon(&section)]);
    let turns = ((to - start) / TAU).ceil() + 1.0;
    let path = crate::shared::sweep::helix(p3(0.0, 0.0, start), 10.0, TAU, 0.0, turns);
    let thread = crate::shared::sweep::sweep(k, &profile, path)?;
    let ends = cylinder(k, p3(0.0, 0.0, from), v3(0.0, 0.0, 1.0), 13.0, to - from)?;
    let thread = crate::bench::common(k, thread, ends)?;
    let thread = crate::bench::turned(k, thread, Axis::new(P3::origin(), v3(0.0, 1.0, 0.0)), 90.0)?;
    crate::bench::moved(k, thread, at.coords)
}

/// `blank` less what the worm's thread on the line through `at` along x (from x = `from` to `to`)
/// passes through as it turns by t about its axis while the gear turns by t/30 about the z axis,
/// t from 0 to 60π: cadrs's cut by a moving tool (`Kernel::cut_swept`) under the two turns (seen
/// from the gear, by t about the worm's axis, then by −t/30 about the z axis). One tooth space
/// is generated, its faces the sheets of the thread's contacts and edges, and cut thirty times
/// round.
pub fn hobbed(k: &mut DefaultKernel, blank: BodyId, at: P3, from: f64, to: f64) -> Result<BodyId, Refusal> {
    use std::f64::consts::{PI, TAU};
    let thread = worm_thread(k, at, from, to)?;
    let motion = SolidMotion::Rigid {
        steps: vec![
            RigidStep::Turn { axis: Axis::new(at, v3(1.0, 0.0, 0.0)), from: 0.0, to: 60.0 * PI },
            RigidStep::Turn { axis: z_axis(), from: 0.0, to: -TAU },
        ],
    };
    cut_swept_exactly(k, blank, thread, &motion)
}
