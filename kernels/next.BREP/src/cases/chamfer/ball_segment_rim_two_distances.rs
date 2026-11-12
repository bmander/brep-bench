//! The part of a ball of radius 5 about the origin below the plane z = 3, the circle where its flat
//! face meets the ball (radius 4 at z = 3) chamfered with two distances, 2 on the flat face and 1
//! on the ball, each a straight line from the edge in the plane square to it: the cut ball cut
//! again by the cone from the circle of radius 2 on the flat face to the circle on the ball 1 in a
//! straight line from the rim.
use crate::solids::boolean::common;
use crate::solids::fillet::distances_ball_first;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{chamfer_edges_asymmetric, make_box_brep, make_sphere_brep, BrepSolid};

const ON_FLAT: f64 = 2.0;
const ON_BALL: f64 = 1.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    let below = make_box_brep(p(-20.0, -20.0, -20.0), 40.0, 40.0, 23.0)?;
    let cut_ball = common(&ball, &below)?;
    // The rim, by a point on it. The kernel's two-distance chamfer sets back its first distance on
    // the edge's first face, each distance a chord in the plane square to the edge.
    let rim = p(0.0, -4.0, 3.0);
    let (d1, d2) = distances_ball_first(&cut_ball, rim, p(0.0, 0.0, 0.0), ON_BALL, ON_FLAT)?;
    Ok(chamfer_edges_asymmetric(&cut_ball, &[rim], None, d1, d2, None)?)
}
