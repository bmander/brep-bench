//! A plate, the disk of radius 8 about the z axis from z = −2 to z = 0, joined to the ball of
//! radius 3 about (0, 0, 1), the concave edge where the plate's top meets the ball rounded to
//! radius 1 by a piece of the torus whose tube has radius 1 about the circle of radius 4 at z = 1;
//! the round then deleted, the top running on inward and the ball down until they meet in the
//! circle of radius √8 at z = 0.
//!
//! The rounded solid is turned whole: its meridian runs out along the plate's floor, up its rim, in
//! along its top to the round, round the round's quarter circle to the ball's equator and up the
//! ball to its pole, so the round is the torus it is. (A workaround: `fillet::roll` refuses the
//! circle where the ball leaves the plate's top.)
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
    let line = |a: [f64; 2], b: [f64; 2]| ProfileEdge::Line { a: [a[0], 0., a[1]], b: [b[0], 0., b[1]] };
    let meridian = vec![
        line([0., -2.], [8., -2.]), line([8., -2.], [8., 0.]), line([8., 0.], [4., 0.]),
        // the round, from the top at radius 4 round to the ball's equator at (3, 1)
        arc([4., 1.], 1., [-PI, -FRAC_PI_2]),
        // the ball, from its equator up to its pole
        arc([0., 1.], 3., [0., FRAC_PI_2]),
        line([0., 4.], [0., -2.]),
    ];
    let section = Profile { origin: [0.; 3], normal: [0., -1., 0.], loops: vec![meridian], names: Vec::new() };
    let b = revolve(&section, [0.; 3], [0., 0., 1.], TAU)?;
    let round = faces_on(&b, |s| matches!(s, Surface::Torus(..)));
    deleting(&b, &round)
}
