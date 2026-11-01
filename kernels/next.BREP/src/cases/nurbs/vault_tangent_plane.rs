//! A block on the square −12 ≤ x, y ≤ 12, its floor at z = 0, under a weighted barrel vault (a
//! rational B-spline surface, degree 1 along x and 2 along y, exactly the arch of radius 15 about
//! the line y = 0, z = 1), cut by a box whose floor is the plane z = 16, tangent to the vault
//! along its crown, and which reaches past the block on every other side.
use crate::solids::p;
use crate::solids::sweep::block_under;
use crate::Refusal;
use brep_kernel::{
    boolean_operation, make_box_brep, BooleanOperation, BooleanOptions, BrepSolid, NurbsSurface,
    Vec4,
};

/// The vault: for x = −12 and x = 12 the points (x, −12, 10), (x, 0, 26) and (x, 12, 10), with
/// weights 1, 3/5 and 1, on the knots (0, 0, 1, 1) along x and (0, 0, 0, 1, 1, 1) along y.
pub fn vault() -> Result<NurbsSurface, String> {
    let row = |x: f64| {
        vec![Vec4::from_point(p(x, -12.0, 10.0), 1.0), Vec4::from_point(p(x, 0.0, 26.0), 0.6),
            Vec4::from_point(p(x, 12.0, 10.0), 1.0)]
    };
    NurbsSurface::new(1, 2, vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        vec![row(-12.0), row(12.0)])
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = block_under(&vault()?)?;
    let above = make_box_brep(p(-20.0, -20.0, 16.0), 40.0, 40.0, 20.0)?;
    Ok(boolean_operation(&block, &above, BooleanOperation::Subtract, &BooleanOptions::default())?)
}
