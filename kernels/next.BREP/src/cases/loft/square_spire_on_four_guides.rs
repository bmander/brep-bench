//! A square spire lofted from the square 18 × 18 at z = 0 to the point (0, 0, 24), its sections
//! squares centred on the z axis with sides parallel to x and y, its corners on four cubic Bézier
//! ridges: the one in the plane x = y has control points (9, 9, 0), (15, 15, 8), (12, 12, 19) and
//! (0, 0, 24), the others its mirror images in the xz and yz planes; each wall ruled by horizontal
//! segments between neighbouring ridges.
//!
//! next.BREP's loft takes no guide curves for its corners (its guided loft rides one spine curve),
//! and closes on no point, so the solid is built from what its walls are. Each section's sides stay
//! parallel to x and y, so the walls facing ±x are the ridge seen along y and its mirror image,
//! meeting at the apex, slid along y, and those facing ±y the same seen along x, slid along x: the
//! solid is the common part of the two extrusions of the regions between them.
use crate::solids::p;
use crate::solids::sweep::between_guides;
use crate::Refusal;
use brep_kernel::{boolean_operation, BooleanOperation, BooleanOptions, BrepSolid};

/// The ridge, (g, g, z), by its g and z.
const RIDGE: [(f64, f64); 4] = [(9.0, 0.0), (15.0, 8.0), (12.0, 19.0), (0.0, 24.0)];

pub fn build() -> Result<BrepSolid, Refusal> {
    let along_y: Vec<_> = RIDGE.iter().map(|&(g, z)| p(g, 0.0, z)).collect();
    let along_x: Vec<_> = RIDGE.iter().map(|&(g, z)| p(0.0, g, z)).collect();
    let walls_x = between_guides(&along_y, p(0.0, 1.0, 0.0), 40.0)?;
    let walls_y = between_guides(&along_x, p(1.0, 0.0, 0.0), 40.0)?;
    Ok(boolean_operation(&walls_x, &walls_y, BooleanOperation::Intersect,
        &BooleanOptions::default())?)
}
