//! `fillet/elliptic-section-cylinder-rim` at a hundredth of its size: a cylinder of radius 0.05
//! on the z axis from z = 0 to z = 0.1, the circle where its side meets its top blended unequally
//! by the kernel's conic blend with two distances: 0.01 across the top (the face nearest the axis's
//! top point) and 0.03 down the side, rho √2 − 1, so its section is the quarter ellipse of
//! semi-axes 0.01 and 0.03. The edge is selected by a point on it off the seam. Built as the
//! original is, every length times 0.01.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges_conic_distances, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cylinder = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 0.05, 0.1)?;
    let rim = p(0.05 * 1f64.cos(), 0.05 * 1f64.sin(), 0.1);
    // 0.01 across the top (the face by the axis's top point), 0.03 down the side; rho √2 − 1, the quarter ellipse.
    Ok(fillet_edges_conic_distances(&cylinder, &[rim], None, p(0.0, 0.0, 0.1), 0.01, 0.03, std::f64::consts::SQRT_2 - 1.0, None)?)
}
