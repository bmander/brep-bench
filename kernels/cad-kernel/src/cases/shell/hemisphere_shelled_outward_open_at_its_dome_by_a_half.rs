//! The upper half of the ball of radius 5 about the origin shelled outward to a wall 0.5 thick,
//! open at its dome: the half ball's flat face, z = 0, moved 0.5 outward along its normal, the
//! moved face extended until it meets the sphere of the opening, which is left as the rim.
//!
//! `offset::shelled` ends an outward wall only at a plane left open ("only a plane cuts one off
//! yet"), so the wall is built from the shell's own piece: the flat face's normal sweep
//! (`offset::normal_sweep`) kept within the opening's sphere (`build::kept_by_solid`).
use crate::sketch::faces_through;
use cad_kernel::build::{kept, kept_by_solid, sphere};
use cad_kernel::offset::normal_sweep;
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;
const WALL: f64 = 0.5;

pub fn build() -> Result<Brep, String> {
    // the ball kept above the plane z = 0
    let body = kept(&sphere([0., 0., 0.], [0., 0., 1.], RADIUS)?, [0., 0., 0.], [0., 0., 1.], 0.)?;
    let flat = faces_through(&body, &[[1., 1., 0.]])?[0];
    let dome = faces_through(&body, &[[3., 0., 4.]])?[0];
    // the flat face swept out along its normal, down to z = −0.5, run on past the sphere
    let wall = normal_sweep(&body, flat, [0., WALL])?;
    // and ended where it meets the sphere of the opening
    kept_by_solid(&wall, &body.faces[dome].surface, true, 0.)
}
