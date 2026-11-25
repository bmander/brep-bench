//! The ball of radius 5 about the origin bounded by the one NURBS face of `geometry/nurbs-sphere`
//! (a 9 × 5 rational net, its first and last rows each collapsed to a pole), grown by 2: its face
//! offset out along its normal through the poles, the ball of radius 7.
//!
//! `offset::grown` reads the rational net as the sphere it lies on (its equation vanishing on a grid
//! of each span), puts the faces on that sphere, and grows them as a ball's.
use crate::cases::geometry::nurbs_sphere;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    grown(&nurbs_sphere::build()?, 2., Join::Round)
}
