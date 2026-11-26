//! The block from (−10, −10, 0) to (10, 10, 10) with a blind bore of radius 3 down its z axis from
//! the top to a flat floor at z = 2, grown by 4, more than the bore's radius: the bore fills, and a
//! dimple stays over its mouth.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

const BORE_RADIUS: f64 = 3.;
const FLOOR: f64 = 2.;
const GROWTH: f64 = 4.;

pub fn build() -> Result<Brep, String> {
    let solid = block([-10., -10., 0.], [10., 10., 10.])?;
    // the drill runs from the floor on 1 past the top, so its top cuts nothing
    let drill = prism(&disc([0., 0., FLOOR], [0., 0., 1.], BORE_RADIUS), 0., 10. - FLOOR + 1.)?;
    grown(&combined(&solid, &drill, Op::Cut, 0.)?, GROWTH, Join::Round)
}
