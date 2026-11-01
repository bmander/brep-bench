//! The regular tetrahedron with corners (3, 3, 3), (3, −3, −3), (−3, 3, −3) and (−3, −3, 3), grown
//! by 1 to its parallel body: its faces moved out, its edges pieces of cylinders and its corners
//! pieces of balls of radius 1.
//!
//! The tetrahedron is the kernel's three-sided pyramid with edges 6√2 and height 4√3, its base
//! centred on the origin, turned so that its axis runs along (1, 1, 1) and moved so that its base
//! is the face opposite (3, 3, 3), centred on (−1, −1, −1).
use crate::solids::offset::parallel;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_pyramid_brep, transform_brep, AffineTransform, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let edge = 6.0 * 2f64.sqrt();
    let pyramid = make_pyramid_brep(p(-edge / 2.0, -edge / 2.0, 0.0), edge, 4.0 * 3f64.sqrt(), 3)?;
    // The pyramid's x, y and z axes go to (−1, 1, 0)/√2, (−1, −1, 2)/√6 and (1, 1, 1)/√3.
    let (a, b, c) = (2f64.sqrt(), 6f64.sqrt(), 3f64.sqrt());
    let turn = AffineTransform::new([
        -1.0 / a, -1.0 / b, 1.0 / c, -1.0,
         1.0 / a, -1.0 / b, 1.0 / c, -1.0,
         0.0,      2.0 / b, 1.0 / c, -1.0,
         0.0,      0.0,     0.0,      1.0,
    ])?;
    let tetrahedron = transform_brep(&pyramid, turn, false)?;
    Ok(parallel(&tetrahedron, 1.0)?)
}
