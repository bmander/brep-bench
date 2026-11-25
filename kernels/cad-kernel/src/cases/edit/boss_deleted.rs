//! A plate, the cylinder of radius 8 on the z axis from z = −4 to z = 0, with a boss standing on
//! it, the cylinder of radius 3 from z = 0 to z = 8; the boss's side and top then deleted, the
//! plate's top closing over the circle of radius 3 at the boss's foot.
//!
//! The bossed plate is both cylinders, `prism`s of `disc`s, joined by `boolean::combined`; the
//! boss's side and top are picked by a point on each and deleted together (`defeature::deleted`).
use crate::sketch::deleted_through;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let plate = prism(&disc([0., 0., 0.], [0., 0., 1.], 8.), -4., 0.)?;
    let boss = prism(&disc([0., 0., 0.], [0., 0., 1.], 3.), 0., 8.)?;
    let bossed = combined(&plate, &boss, Op::Union, 0.)?;
    deleted_through(&bossed, &[[3., 0., 4.], [1., 0., 8.]])
}
