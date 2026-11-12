//! A slab lofted through rectangles from z = 0 to z = 24, centred on the z axis with sides parallel
//! to x and y, its corners on four cubic Bézier guides: the one through the corner with x and y
//! positive has control points (12, 3.2, 0), (18, −2.8, 8), (15, 0.2, 19) and (3, 12.2, 24), the
//! others its mirror images in the xz and yz planes; each wall ruled by horizontal segments
//! parallel to x or y between neighbouring guides.
//!
//! The kernel's loft along guides (`loft_profile_brep_on_guides`): the rectangle at z = 0 and the
//! one at z = 24, and the four guides through their corners, each wall the patch between two
//! neighbouring guides and the two sides they join.
use crate::solids::sweep::bezier;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{loft_profile_brep_on_guides, BrepSolid};

/// The guide through the corner with x and y positive.
const GUIDE: [(f64, f64, f64); 4] =
    [(12.0, 3.2, 0.0), (18.0, -2.8, 8.0), (15.0, 0.2, 19.0), (3.0, 12.2, 24.0)];

pub fn build() -> Result<BrepSolid, Refusal> {
    // the corners in turn, by the signs of their x and y
    let signs = [(1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), (1.0, -1.0)];
    let corner = |k: usize, i: usize| p(signs[k].0 * GUIDE[i].0, signs[k].1 * GUIDE[i].1, GUIDE[i].2);
    let section = |i: usize| polygon(&[corner(0, i), corner(1, i), corner(2, i), corner(3, i)]);
    let guides = (0..4)
        .map(|k| bezier(&[corner(k, 0), corner(k, 1), corner(k, 2), corner(k, 3)]))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(loft_profile_brep_on_guides(&section(0)?, &section(3)?, &guides)?)
}
