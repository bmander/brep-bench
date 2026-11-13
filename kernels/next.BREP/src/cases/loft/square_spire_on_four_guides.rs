//! A square spire lofted from the square 18 × 18 at z = 0 to the point (0, 0, 24), its sections
//! squares centred on the z axis with sides parallel to x and y, its corners on four cubic Bézier
//! ridges: the one in the plane x = y has control points (9, 9, 0), (15, 15, 8), (12, 12, 19) and
//! (0, 0, 24), the others its mirror images in the xz and yz planes; each wall ruled by horizontal
//! segments between neighbouring ridges.
//!
//! The kernel's loft along guides (`loft_profile_brep_on_guides`): the square at z = 0, the point
//! (0, 0, 24) as a section of four curves of no length, and the four ridges, each wall the patch
//! between two neighbouring ridges and the side they stand on, closed on the point.
use crate::solids::sweep::bezier;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{loft_profile_brep_on_guides, BrepSolid};

/// The ridge, (g, g, z), by its g and z.
const RIDGE: [(f64, f64); 4] = [(9.0, 0.0), (15.0, 8.0), (12.0, 19.0), (0.0, 24.0)];

pub fn build() -> Result<BrepSolid, Refusal> {
    // the corners in turn, by the signs of their x and y
    let signs = [(1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), (1.0, -1.0)];
    let corner = |k: usize, i: usize| p(signs[k].0 * RIDGE[i].0, signs[k].1 * RIDGE[i].0, RIDGE[i].1);
    let section = |i: usize| polygon(&[corner(0, i), corner(1, i), corner(2, i), corner(3, i)]);
    let ridges = (0..4)
        .map(|k| bezier(&[corner(k, 0), corner(k, 1), corner(k, 2), corner(k, 3)]))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(loft_profile_brep_on_guides(&section(0)?, &section(3)?, &ridges)?)
}
