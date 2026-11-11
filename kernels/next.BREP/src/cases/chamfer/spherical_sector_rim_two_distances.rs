//! The spherical sector of the ball of radius 5 about the origin inside the cone with its apex at
//! the origin about the z axis, cos α = 3/5, its rim (the circle of radius 4 at z = 3) chamfered
//! with two distances, 2 down the cone and 1 on the ball, each a straight line from the edge in the
//! plane square to it: the sector cut by the cone from the circle 2 down the side from the rim to
//! the circle on the ball 1 in a straight line from it.
//!
//! The sector is its section turned a whole turn about the z axis: the line from the origin to
//! (4, 0, 3), the arc of radius 5 from there to the pole (0, 0, 5), and the axis back down.
use crate::solids::fillet::distances_ball_first;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{chamfer_edges_asymmetric, make_arc, make_line, revolve_profile_brep, BrepSolid};
use std::f64::consts::{FRAC_PI_2, TAU};

const DOWN_CONE: f64 = 2.0;
const ON_BALL: f64 = 1.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let (o, x, z) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let section = [
        make_line(o, p(4.0, 0.0, 3.0))?,
        make_arc(o, x, z, 5.0, 3f64.atan2(4.0), FRAC_PI_2)?,
        make_line(p(0.0, 0.0, 5.0), o)?,
    ];
    let sector = revolve_profile_brep(&section, o, z, TAU)?;
    // The rim, by a point on it. The kernel's two-distance chamfer sets back its first distance on
    // the edge's first face, each distance a chord in the plane square to the edge.
    let rim = p(0.0, -4.0, 3.0);
    let (d1, d2) = distances_ball_first(&sector, rim, o, ON_BALL, DOWN_CONE)?;
    Ok(chamfer_edges_asymmetric(&sector, &[rim], None, d1, d2, None)?)
}
