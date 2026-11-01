//! A slab lofted through rectangles from z = 0 to z = 24, centred on the z axis with sides parallel
//! to x and y, its corners on four cubic Bézier guides: the one through the corner with x and y
//! positive has control points (12, 3.2, 0), (18, −2.8, 8), (15, 0.2, 19) and (3, 12.2, 24), the
//! others its mirror images in the xz and yz planes; each wall ruled by horizontal segments
//! parallel to x or y between neighbouring guides.
//!
//! next.BREP's loft takes no guide curves for its corners (its guided loft rides one spine curve),
//! so the solid is built from what its walls are. Each section's sides stay parallel to x and y,
//! so the walls facing ±x are the guide seen along y and its mirror image, each slid along y, and
//! those facing ±y the guide seen along x and its mirror, slid along x: the solid is the common
//! part of the two extrusions of the regions between them.
use crate::solids::p;
use crate::solids::sweep::between_guides;
use crate::Refusal;
use brep_kernel::{boolean_operation, BooleanOperation, BooleanOptions, BrepSolid};

/// The guide through the corner with x and y positive.
const GUIDE: [(f64, f64, f64); 4] =
    [(12.0, 3.2, 0.0), (18.0, -2.8, 8.0), (15.0, 0.2, 19.0), (3.0, 12.2, 24.0)];

pub fn build() -> Result<BrepSolid, Refusal> {
    let along_y: Vec<_> = GUIDE.iter().map(|&(x, _, z)| p(x, 0.0, z)).collect();
    let along_x: Vec<_> = GUIDE.iter().map(|&(_, y, z)| p(0.0, y, z)).collect();
    let walls_x = between_guides(&along_y, p(0.0, 1.0, 0.0), 40.0)?;
    let walls_y = between_guides(&along_x, p(1.0, 0.0, 0.0), 40.0)?;
    Ok(boolean_operation(&walls_x, &walls_y, BooleanOperation::Intersect,
        &BooleanOptions::default())?)
}
