//! A plate 12 thick whose outline is six lobes (arcs of radius 8 about the points 40 from the z
//! axis at 0°, 60°, …) and six hollows between them (arcs of radius 30 touching the lobes beside
//! them from outside), one tangent loop of twelve arcs, both rims rounded to radius 1.
//!
//! The outline is the twelve arcs, each lobe's from where it touches the hollow before it to
//! where it touches the hollow after it; the plate is that region extruded; then cadrs's Fillet
//! on the 24 edges of its two rims, both rims in one feature.
use crate::bench::{arc, extrude, p2};
use crate::shared::fillet::{edges_at_height, fillet};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};
use std::f64::consts::PI;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let (lobe_r, hollow_r) = (8.0, 30.0);
    let at = |r: f64, deg: f64| (r * deg.to_radians().cos(), r * deg.to_radians().sin());
    let reach = 20.0 * 3f64.sqrt() + 6.0 * 29f64.sqrt();
    let lobe = |i: usize| at(40.0, 60.0 * i as f64);
    let hollow = |i: usize| at(reach, 30.0 + 60.0 * i as f64);
    // Where lobe i touches hollow j: 8 along the line between their centres.
    let touch = |l: (f64, f64), h: (f64, f64)| {
        let f = lobe_r / (lobe_r + hollow_r);
        (l.0 + (h.0 - l.0) * f, l.1 + (h.1 - l.1) * f)
    };
    let angle = |c: (f64, f64), p: (f64, f64)| (p.1 - c.1).atan2(p.0 - c.0);
    let mut curves = Vec::new();
    for i in 0..6 {
        let (l, before, after) = (lobe(i), hollow((i + 5) % 6), hollow(i));
        let (a, b) = (angle(l, touch(l, before)), angle(l, touch(l, after)));
        let sweep = (b - a).rem_euclid(2.0 * PI);
        curves.push(arc(p2(l.0, l.1), lobe_r, a.to_degrees(), sweep.to_degrees()));
        // The hollow after it, run clockwise about its centre to the next lobe.
        let next = lobe((i + 1) % 6);
        let (c, d) = (angle(after, touch(l, after)), angle(after, touch(next, after)));
        let back = -(c - d).rem_euclid(2.0 * PI);
        curves.push(arc(p2(after.0, after.1), hollow_r, c.to_degrees(), back.to_degrees()));
    }
    let plate = extrude(k, &Profile::of(Plane::xy(0.0), vec![Loop::new(curves)]), 12.0)?;
    let mut rims = edges_at_height(k, plate, 0.0)?;
    rims.extend(edges_at_height(k, plate, 12.0)?);
    fillet(k, plate, &rims, 1.0)
}
