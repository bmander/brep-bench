//! The upper half of a ball of radius 5 about the origin, the circle where its flat face meets the
//! ball chamfered with two distances, 2 on the flat face and 1 on the ball, each a straight line
//! from the edge in the plane square to it: the hemisphere cut by the cone from the circle of
//! radius 3 on the flat face to the circle on the ball 1 in a straight line from the rim.
use crate::solids::boolean::common;
use crate::solids::fillet::distances_ball_first;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{chamfer_edges_asymmetric, make_box_brep, make_sphere_brep, BrepSolid};

const ON_FLAT: f64 = 2.0;
const ON_BALL: f64 = 1.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    let upper = make_box_brep(p(-20.0, -20.0, 0.0), 40.0, 40.0, 20.0)?;
    let hemisphere = common(&ball, &upper)?;
    // The rim, by a point on it. The kernel's two-distance chamfer sets back its first distance on
    // the edge's first face, each distance a chord in the plane square to the edge.
    let rim = p(0.0, -5.0, 0.0);
    let (d1, d2) = distances_ball_first(&hemisphere, rim, p(0.0, 0.0, 0.0), ON_BALL, ON_FLAT)?;
    Ok(chamfer_edges_asymmetric(&hemisphere, &[rim], None, d1, d2, None)?)
}
