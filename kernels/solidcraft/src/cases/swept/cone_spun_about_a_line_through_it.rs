//! The solid a cone fills as it is spun 150° about the z axis, right-handed. Its axis is the line
//! through the origin along w = (cos 30°, 0, sin 30°); its point is at −2w and it opens at 25°
//! from its axis to a flat base at 5w, of radius 7 tan 25°.
//!
//! solidcraft-kernel has no sweep of a solid, so the turn is built by elementary geometry. The
//! cone's axis meets the z axis at the origin, so every point of its base's rim is one distance
//! from it, R = √(5² + (7 tan 25°)²). Fill the cone out to the ball of radius R about the origin:
//! K′, the cone with its base replaced by that sphere's cap. A circle about the z axis stays on a
//! sphere about the origin; beyond the base it meets the cone in two arcs mirrored in the xz
//! plane, and the cap fills the gap between them, which is narrower than the turn, so K′ sweeps
//! what the cone does. K′ is symmetric in the xz plane and meets every circle about the z axis in
//! arcs about its point at 0° and/or at 180°, or holds it whole; turned by 150°, an arc about 0° is
//! passed over where K′ starts, the 150° on from 0° and where it stops, and one about 180°
//! likewise from 180°. So the sweep is K′ where it starts and where it stops, with K′'s section by
//! the xz plane on the side x ≥ 0 revolved from 0° to 150° and on the side x ≤ 0 from 180° to
//! 330°: cones, spheres and planes.
use super::drum_swung_partway_into_ring_sector::turned;
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> R<Body> {
    spun(1.0)
}

/// The sweep with every length `s` times as long.
pub fn spun(s: f64) -> R<Body> {
    let (s30, c30) = 30f64.to_radians().sin_cos();
    let w = v3(c30, 0.0, s30);
    let u = v3(-s30, 0.0, c30); // square to w, in the xz plane
    let (tip, base) = (2.0 * s, 5.0 * s);
    let rb = 7.0 * s * 25f64.to_radians().tan();
    let big = (base * base + rb * rb).sqrt();
    // K′, revolved about w: its section in (out from w along u, along w) from the point out to
    // the rim at (rb, 5), the sphere's arc up to the axis at R, and the axis back
    let filled = vec![
        Seg2::Line { a: v2(0.0, -tip), b: v2(rb, base) },
        Seg2::Arc { center: v2(0.0, 0.0), radius: big, start: base.atan2(rb), sweep: FRAC_PI_2 - base.atan2(rb) },
        Seg2::Line { a: v2(0.0, big), b: v2(0.0, -tip) },
    ];
    let frame = Plane { origin: Vec3::ZERO, x: u, y: w };
    let filled = Region2 { outer: Loop2 { segs: filled }, holes: vec![] };
    let start = one(solidcraft_kernel::revolve(&frame, &[filled], Vec2::ZERO, Vec2::Y, TAU)?)?;
    // K′'s section by the xz plane, in (x, z): the point a = −2w, the rims 5w ± rb·u, the
    // sphere's arc between them; the z axis crosses its sides at `top` and `bottom`
    let at = |along: f64, out: f64| w * along + u * out;
    let a = at(-tip, 0.0);
    let (rim_up, rim_down) = (at(base, rb), at(base, -rb));
    let cross = |q: Vec3| a + (q - a) * (-a.x / (q.x - a.x)); // where the side from a to q meets x = 0
    let (top, bottom) = (cross(rim_up), cross(rim_down));
    let line = |p: Vec3, q: Vec3| Seg2::Line { a: v2(p.x, p.z), b: v2(q.x, q.z) };
    let angle = |q: Vec3| q.z.atan2(q.x);
    let outer = vec![
        line(bottom, rim_down),
        Seg2::Arc { center: v2(0.0, 0.0), radius: big, start: angle(rim_down), sweep: angle(rim_up) - angle(rim_down) },
        line(rim_up, top),
        line(top, bottom),
    ];
    let ahead = revolve_z(Region2 { outer: Loop2 { segs: outer }, holes: vec![] }, 150.0)?;
    // the side x ≤ 0, the triangle a, bottom, top, drawn mirrored at 0° and turned to 180°
    let inner = polygon(&[(-a.x, a.z), (0.0, top.z), (0.0, bottom.z)]);
    let behind = turned(&revolve_z(inner, 150.0)?, 180.0)?;
    let end = turned(&start, 150.0)?;
    // The kernel's boolean refuses to unite `ahead` with K′ where it starts or stops: they
    // overlap on the sphere, and touch along the section's sides. So `ahead` is first joined to
    // the parts of K′ that only meet it on its end faces, K′ less its quarter x ≥ 0, y ≥ 0 where
    // it starts and less its quarter x ≥ 0, y ≤ 0 where it stops; the rest of K′ comes with
    // `behind`, which the kernel unites with it.
    let far = 2.0 * big;
    let first = cut(&start, &block((0.0, 0.0, -far), (far, far, far))?)?;
    let last = turned(&cut(&start, &block((0.0, -far, -far), (far, 0.0, far))?)?, 150.0)?;
    let ahead = union(&union(&ahead, &first)?, &last)?;
    union(&union(&union(&behind, &start)?, &end)?, &ahead)
}
