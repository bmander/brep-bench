//! A square column from z = 0 to z = 40, its section at height z the square of side 30 − z/2
//! centred on the z axis and turned about it, right-handed, by 4.5° for every unit of height:
//! from 30 × 30 at the foot to 10 × 10 at the top, half a turn on the way, its corners on four
//! conical spirals.
//!
//! Refused: arris's `loft` takes no guide curves (it joins planar sections by ruled spans or by
//! one polynomial through at most nine of them), and its `sweep` carries a profile rigidly, with
//! no law to scale it, along paths of lines, arcs, helices and NURBS curves, with no conical
//! spiral. The section both turns and shrinks, so no rigid motion carries it, and each wall,
//! whose horizontal rulings turn in proportion to height, is no surface a loft through finitely
//! many of its sections makes: lofting through samples of it would only approximate it.
use crate::Refusal;
use arris::topo::{Body, Model};

pub fn build(_: &mut Model) -> Result<Body, Refusal> {
    Err("arris's loft takes no guide curves and its sweep moves a profile rigidly, with no \
         scaling law and no conical-spiral path; the square both turns and tapers, so its walls, \
         ruled by horizontal lines turning in proportion to height, are not a surface it can \
         build (a loft through sampled sections would only approximate them)"
        .into())
}
