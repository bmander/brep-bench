//! A cone, its base of radius 5 at z = 0 and its apex 10 above it on the z axis, less a cone of
//! base radius 4.999 with the same base plane and the same apex: a conical shell 0.001 thick at
//! its base, thinning to nothing at the apex the two share, its base a thin annulus.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::cone;
use cad_kernel::topo::Brep;

const UP: [f64; 3] = [0., 0., 1.];
const RADIUS: f64 = 5.;
const INNER: f64 = 4.999;
const HEIGHT: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    // each from its base up to its apex, a radius of nothing there
    let outer = cone([0., 0., 0.], UP, RADIUS, 0., HEIGHT)?;
    let inner = cone([0., 0., 0.], UP, INNER, 0., HEIGHT)?;
    Ok(combined(&outer, &inner, Op::Cut, 0.)?)
}
