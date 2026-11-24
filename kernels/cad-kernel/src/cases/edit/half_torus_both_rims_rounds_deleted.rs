//! The half above z = 0 of the ring torus about the z axis whose tube has radius 2 about the circle
//! of radius 6, its flat the ring 4 ≤ r ≤ 8 at z = 0, both rims rounded to radius 1/2 by pieces of
//! the tori about the circles of radius 6 + √2 and 6 − √2 at z = 1/2; both rounds then deleted
//! together, the flat running on outward and inward and the torus down until they meet in the
//! circles of radius 8 and 4 at z = 0.
//!
//! The rounded half torus is turned whole: its meridian runs out along the flat from the inner
//! round to the outer, round the outer round's arc onto the tube's circle, over the top of the tube
//! and down the inner round's arc, so each round is the torus it is.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_on};
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::{Frame, Surface};
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, PI, TAU};

pub fn build() -> Result<Brep, String> {
    // arcs in the plane y = 0, their angles from +x toward +z
    let arc = |c: [f64; 2], r: f64, span: [f64; 2]| ProfileEdge::Arc {
        frame: Frame::new([c[0], 0., c[1]], [0., -1., 0.], [1., 0., 0.]), r, span: Some(span) };
    let s = 2f64.sqrt();
    // each round's middle 1.5 from the tube's, so where it meets the tube lies on the line between
    let a = 0.5f64.atan2(s);
    let meridian = vec![
        ProfileEdge::Line { a: [6. - s, 0., 0.], b: [6. + s, 0., 0.] },
        arc([6. + s, 0.5], 0.5, [-FRAC_PI_2, a]),
        arc([6., 0.], 2., [a, PI - a]),
        arc([6. - s, 0.5], 0.5, [PI - a, 3. * FRAC_PI_2]),
    ];
    let section = Profile { origin: [6., 0., 0.], normal: [0., -1., 0.], loops: vec![meridian], names: Vec::new() };
    let b = revolve(&section, [0.; 3], [0., 0., 1.], TAU)?;
    // the rounds: the tori of the smaller tube
    let rounds = faces_on(&b, &|s| matches!(s, Surface::Torus(_, _, r) if *r < 1.));
    deleting(&b, &rounds)
}
