//! A plate, the disk of radius 8 about the z axis from z = −2 to z = 0, joined to the ball of
//! radius 3 about (0, 0, 1), the groove where they meet filled by a bevel, the frustum from radius
//! 4 at z = 0 to radius 3 at z = 1; the bevel's cone then deleted, the plate's top running on
//! inward and the ball down from its equator until they meet in the circle of radius √8 at z = 0.
//!
//! The beveled solid is the plate, a `prism` of a `disc`, the ball, a `sphere` (`kept` above the
//! plate's top, which holds the rest of it, so as not to graze its bottom at (0, 0, −2)), and the
//! bevel, the frustum `build::cone`, joined by `boolean::combined`; the bevel is picked by a point
//! on it and deleted (`defeature::deleted`).
use crate::sketch::deleted_through;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, disc, kept, prism, sphere};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let plate = prism(&disc([0., 0., 0.], UP, 8.), -2., 0.)?;
    let ball = kept(&sphere([0., 0., 1.], UP, 3.)?, [0., 0., 0.], UP, 0.)?;
    let bevel = cone([0., 0., 0.], UP, 4., 3., 1.)?;
    let beveled = combined(&combined(&plate, &ball, Op::Union, 0.)?, &bevel, Op::Union, 0.)?;
    deleted_through(&beveled, &[[3.5, 0., 0.5]])
}
