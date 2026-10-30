//! A block (−10, −10, 0)–(10, 10, 10) united with a cone on z, its apex at the origin on the block's
//! bottom face and its base the disc of radius 5 at z = 15.
use crate::sketch::{block, polygon};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::revolve;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const HALF_SIDE: f64 = 10.;
const HEIGHT: f64 = 10.;
const CONE_RADIUS: f64 = 5.;
const CONE_TOP: f64 = 15.;

pub fn build() -> Result<Brep, String> {
    let stock = block([-HALF_SIDE, -HALF_SIDE, 0.], [HALF_SIDE, HALF_SIDE, HEIGHT])?;
    // workaround: no cone primitive
    // the cone's meridian in the xz plane, the axis one of its sides, turned once about z: its tip
    // touches the block's bottom face at a single point
    let meridian = polygon(&[[0., 0., 0.], [CONE_RADIUS, 0., CONE_TOP], [0., 0., CONE_TOP]]);
    let cone = revolve(&meridian, [0., 0., 0.], [0., 0., 1.], TAU)?;
    Ok(combined(&stock, &cone, Op::Union, 0.)?)
}
