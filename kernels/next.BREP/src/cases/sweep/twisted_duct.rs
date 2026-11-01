//! A solid duct along the y axis from the square A (10, 0, −10), B (−10, 0, −10), C (−10, 0, 10),
//! D (10, 0, 10) at y = 0 to the diamond A′ (0, 40, −12), B′ (−12, 40, 0), C′ (0, 40, 12),
//! D′ (12, 40, 0) at y = 40: a two-section loft, each side of the square railed to the side of
//! the diamond with the same letters, so each wall is the ruled surface between them.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{loft_profile_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let square = polygon(&[p(10.0, 0.0, -10.0), p(-10.0, 0.0, -10.0), p(-10.0, 0.0, 10.0),
        p(10.0, 0.0, 10.0)])?;
    let diamond = polygon(&[p(0.0, 40.0, -12.0), p(-12.0, 40.0, 0.0), p(0.0, 40.0, 12.0),
        p(12.0, 40.0, 0.0)])?;
    Ok(loft_profile_brep(&[square, diamond])?)
}
