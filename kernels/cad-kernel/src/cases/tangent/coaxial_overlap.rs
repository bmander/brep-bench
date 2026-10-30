//! Two cylinders of radius 5 on the z axis, from z = 0 to 10 and from z = 6 to 16, united: their
//! walls coincide between z = 6 and z = 10.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::Rigid;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_PI_2;

const UP: [f64; 3] = [0., 0., 1.];
const RADIUS: f64 = 5.;
const LENGTH: f64 = 10.;
const SECOND_FOOT: f64 = 6.;

pub fn build() -> Result<Brep, String> {
    let first = prism(&disc([0., 0., 0.], UP, RADIUS), 0., LENGTH)?;
    // a disc's circle is seamed where its frame's x points, the same for both: the second turned a
    // quarter about z, so the coincident walls are not split in the same place
    let second = prism(&disc([0., 0., SECOND_FOOT], UP, RADIUS), 0., LENGTH)?
        .moved(&Rigid::turn([0., 0., 0.], UP, FRAC_PI_2));
    Ok(combined(&first, &second, Op::Union, 0.)?)
}
