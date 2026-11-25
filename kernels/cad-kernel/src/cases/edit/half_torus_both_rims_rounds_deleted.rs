//! The half above z = 0 of the ring torus about the z axis whose tube has radius 2 about the circle
//! of radius 6, its flat the ring 4 ≤ r ≤ 8 at z = 0, both rims rounded to radius 1/2 by pieces of
//! the tori about the circles of radius 6 + √2 and 6 − √2 at z = 1/2; both rounds then deleted
//! together, the flat running on outward and inward and the torus down until they meet in the
//! circles of radius 8 and 4 at z = 0.
//!
//! The rounded half torus is its half section in the plane y = 0 turned once about the z axis by
//! `build::revolve`, which lays each arc on a torus: the flat from radius 6 − √2 to 6 + √2, the
//! outer round's arc about (6 + √2, 0, 1/2) up to where it meets the tube, the tube's arc of radius
//! 2 about (6, 0, 0) over the top, the inner round's arc about (6 − √2, 0, 1/2) back down to the
//! flat. (`fillet::roll` refuses the inner rim of the torus `kept` above z = 0: an edge whose
//! parameters do not increase.) The two rounds, the faces on tori of tube radius 1/2, are deleted
//! together (`defeature::deleted`).
use crate::sketch::deleted_where;
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::{Frame, Surface, V};
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, PI, SQRT_2, TAU};

pub fn build() -> Result<Brep, String> {
    let at = |r: f64, z: f64| -> V { [r, 0., z] };
    // arcs in the plane y = 0, angles from +r toward +z
    let about = |r: f64, z: f64| Frame::new(at(r, z), [0., -1., 0.], [1., 0., 0.]);
    // each round's centre is 3/2 from the tube's, at (±√2, 1/2) from it: they meet along that line
    let meet = 0.5f64.atan2(SQRT_2);
    let edges = vec![
        ProfileEdge::Line { a: at(6. - SQRT_2, 0.), b: at(6. + SQRT_2, 0.) },
        ProfileEdge::Arc { frame: about(6. + SQRT_2, 0.5), r: 0.5, span: Some([-FRAC_PI_2, meet]) },
        ProfileEdge::Arc { frame: about(6., 0.), r: 2., span: Some([meet, PI - meet]) },
        ProfileEdge::Arc { frame: about(6. - SQRT_2, 0.5), r: 0.5, span: Some([PI - meet, 1.5 * PI]) },
    ];
    let section = Profile { origin: at(0., 0.), normal: [0., -1., 0.], loops: vec![edges], names: Vec::new() };
    let rounded = revolve(&section, [0., 0., 0.], [0., 0., 1.], TAU)?;
    deleted_where(&rounded, |s| matches!(s, Surface::Torus(_, _, r) if *r < 1.))
}
