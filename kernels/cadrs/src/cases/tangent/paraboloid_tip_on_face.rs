//! A block from (−10, −10, 0) to (10, 10, 10) united with a solid paraboloid x² + y² ≤ 8z up to
//! z = 12.5, its vertex touching the middle of the block's bottom face. The paraboloid is the
//! parabola x² = 8z of the xz plane from its vertex to (10, 12.5), drawn as the quadratic Bézier
//! it is (its tangents meet at (5, 0)), closed by the top edge and the axis and turned a whole
//! turn about z.
use crate::bench::{block, p3, union};
use crate::shared::geometry::{turned_about_z, Section};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Plane};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(-10.0, -10.0, 0.0), p3(10.0, 10.0, 10.0))?;
    let section = Section::on(Plane::xz(0.0))
        .bezier(&[(0.0, 0.0), (5.0, 0.0), (10.0, 12.5)], &[])
        .line((10.0, 12.5), (0.0, 12.5))
        .line((0.0, 12.5), (0.0, 0.0));
    let paraboloid = turned_about_z(k, &section)?;
    union(k, b, paraboloid)
}
