//! The cube from the origin to (10, 10, 10), the three edges meeting at its corner (10, 10, 10)
//! rounded to radius 2, three quarter cylinders and the eighth of a ball of radius 2 about (8, 8,
//! 8) where they meet; all four then deleted, the faces x = 10, y = 10 and z = 10 running on until
//! they meet in the three edges and the corner.
//!
//! The rounded cube as cad-kernel composes rounds meeting at a corner
//! (`fillet::notched_cube_inside_corner`'s way, at a convex corner): each edge's round rolled
//! against the cube (`fillet::roll`), cut short at the corner's section through the ball's centre
//! square to the edge (`build::cut_before`), and cut away; then the corner's piece, its cell less
//! the ball (`fillet::corner`), cut away. Every face of it not a plane, the three quarter cylinders
//! and the ball's eighth, is deleted together (`defeature::deleted`).
use crate::sketch::{block, named};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::cut_before;
use cad_kernel::defeature::deleted;
use cad_kernel::fillet::{corner, roll};
use cad_kernel::geom::{tolerance, Surface, V};
use cad_kernel::topo::Brep;

const SIDE: f64 = 10.;
const ROUND: f64 = 2.;
/// The ball that closes the corner: `ROUND` in from each of the three faces.
const CENTRE: V = [SIDE - ROUND; 3];

pub fn build() -> Result<Brep, String> {
    // (every face named apart: a roll reads faces of one name as pieces of one face)
    let cube = named(block([0.; 3], [SIDE; 3])?);
    let tol = tolerance(cube.size());
    let mut solid = cube.clone();
    // each edge at the corner, along the axis `k` toward it, picked by its middle
    for k in 0..3 {
        let mut at = [SIDE; 3];
        at[k] = 0.5 * SIDE;
        let round = roll(&cube, at, ROUND, tol).map_err(|e| format!("the edge at {at:?}: {e}"))?;
        // the run cut short at the corner's section, the plane through the centre square to it
        let mut along = [0.; 3];
        along[k] = -1.;
        let run = cut_before(&round.piece, CENTRE, along, 2. * ROUND, 0.)
            .map_err(|e| format!("the round at {at:?} cut at the corner: {e}"))?;
        let op = if round.concave { Op::Union } else { Op::Cut };
        solid = combined(&solid, &run, op, 0.).map_err(|e| format!("the round at {at:?} applied: {e}"))?;
    }
    // the corner: from each face toward the ball is into the cube, −x, −y and −z
    let cell = corner([SIDE; 3], CENTRE, ROUND, [[-1., 0., 0.], [0., -1., 0.], [0., 0., -1.]], tol)?;
    let rounded = combined(&solid, &cell, Op::Cut, 0.).map_err(|e| format!("the corner applied: {e}"))?;
    let rounds: Vec<usize> = (0..rounded.faces.len())
        .filter(|&f| !matches!(rounded.faces[f].surface, Surface::Plane(_)))
        .collect();
    if rounds.len() != 4 {
        return Err(format!("the rounded corner has {} faces not planes, not 4", rounds.len()));
    }
    Ok(deleted(&rounded, &rounds, tolerance(rounded.size()))?)
}
