//! A bar of square section 20 × 20 bent along the planar cubic Bézier curve with control points
//! (0, 0, 0), (16, 12, 0), (18, 26, 0) and (12, 34, 0), its section square to the curve with two
//! sides horizontal, its ends flat.
//!
//! The square is drawn where the bar starts, square to the curve's first direction (4, 3, 0), and
//! carried along the curve by the path's own motion (`SectionPlacement::Rigid`, the Sweep
//! feature's `pathAlign`), which on a planar curve keeps its horizontal sides horizontal.
use crate::solids::{p, polygon};
use crate::solids::sweep::bezier;
use crate::Refusal;
use brep_kernel::{sweep_profile_along_chain, BrepSolid, SectionPlacement, SweepPath};

pub fn build() -> Result<BrepSolid, Refusal> {
    let path = bezier(&[p(0.0, 0.0, 0.0), p(16.0, 12.0, 0.0), p(18.0, 26.0, 0.0),
        p(12.0, 34.0, 0.0)])?;
    // corners 10 to the left (−3, 4, 0)/5 or right of the start and 10 above or below it
    let (l, h) = (p(-6.0, 8.0, 0.0), 10.0);
    let square = polygon(&[
        p(l.x, l.y, -h), p(-l.x, -l.y, -h), p(-l.x, -l.y, h), p(l.x, l.y, h),
    ])?;
    let path = SweepPath::new(vec![path], vec![])?;
    Ok(sweep_profile_along_chain(&square, &path, 0.0, None, None, SectionPlacement::Rigid, "")?)
}
