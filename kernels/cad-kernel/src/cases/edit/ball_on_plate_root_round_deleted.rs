//! A plate, the disk of radius 8 about the z axis from z = −2 to z = 0, joined to the ball of
//! radius 3 about (0, 0, 1), the concave edge where the plate's top meets the ball rounded to
//! radius 1 by a piece of the torus whose tube has radius 1 about the circle of radius 4 at z = 1;
//! the round then deleted, the top running on inward and the ball down until they meet in the
//! circle of radius √8 at z = 0.
//!
//! The rounded solid is its half section in the plane y = 0 turned once about the z axis by
//! `build::revolve`, which lays the ball's arc on a sphere and the round's on a torus: the plate's
//! bottom, its rim and its top in to the round, the round's quarter about (4, 0, 1) in to the ball,
//! the ball's arc about (0, 0, 1) up to its pole, the axis. (`fillet::roll` refuses this concave
//! circle between a plane and a sphere: the strip on one side winds round its surface on one edge
//! and not the other.) The round, the face on a torus, is deleted (`defeature::deleted`).
use crate::sketch::deleted_where;
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::{Frame, Surface, V};
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, PI, TAU};

pub fn build() -> Result<Brep, String> {
    let at = |r: f64, z: f64| -> V { [r, 0., z] };
    // arcs in the plane y = 0, angles from +r toward +z
    let about = |r: f64, z: f64| Frame::new(at(r, z), [0., -1., 0.], [1., 0., 0.]);
    let edges = vec![
        ProfileEdge::Line { a: at(0., -2.), b: at(8., -2.) },
        ProfileEdge::Line { a: at(8., -2.), b: at(8., 0.) },
        ProfileEdge::Line { a: at(8., 0.), b: at(4., 0.) },
        // the round, from the plate's top at (4, 0) round the corner side to the ball at (3, 1)
        ProfileEdge::Arc { frame: about(4., 1.), r: 1., span: Some([-PI, -FRAC_PI_2]) },
        ProfileEdge::Arc { frame: about(0., 1.), r: 3., span: Some([0., FRAC_PI_2]) },
        ProfileEdge::Line { a: at(0., 4.), b: at(0., -2.) },
    ];
    let section = Profile { origin: at(0., 0.), normal: [0., -1., 0.], loops: vec![edges], names: Vec::new() };
    let rounded = revolve(&section, [0., 0., 0.], [0., 0., 1.], TAU)?;
    deleted_where(&rounded, |s| matches!(s, Surface::Torus(..)))
}
