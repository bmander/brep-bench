//! The lens common to two balls of radius 5 about (0, 0, 3) and (0, 0, −3), its rim (the circle of
//! radius 4 at z = 0) chamfered with two distances, 2 on its upper face (on the ball about
//! (0, 0, −3)) and 1 on its lower face, each a straight line from the edge in the plane square to
//! it: the lens cut by the cone between the two circles those chords reach.
use crate::solids::boolean::common;
use crate::solids::fillet::distances_ball_first;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{chamfer_edges_asymmetric, make_sphere_brep, BrepSolid};

const ON_UPPER: f64 = 2.0;
const ON_LOWER: f64 = 1.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let above = make_sphere_brep(p(0.0, 0.0, 3.0), 5.0, p(0.0, 0.0, 1.0))?;
    let below = make_sphere_brep(p(0.0, 0.0, -3.0), 5.0, p(0.0, 0.0, 1.0))?;
    let lens = common(&above, &below)?;
    // The rim, by a point on it. The kernel's two-distance chamfer sets back its first distance on
    // the edge's first face, each distance a chord in the plane square to the edge; the upper face
    // lies on the ball about (0, 0, −3).
    let rim = p(0.0, -4.0, 0.0);
    let (d1, d2) = distances_ball_first(&lens, rim, p(0.0, 0.0, -3.0), ON_UPPER, ON_LOWER)?;
    Ok(chamfer_edges_asymmetric(&lens, &[rim], None, d1, d2, None)?)
}
