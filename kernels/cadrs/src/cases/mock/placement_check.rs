//! The placement's two halves agree: a box placed by the kernel's transforms has the bounding box
//! of its corners carried by `point`.
use crate::bench::{block, p3};
use crate::shared::mock::{place, point};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Kernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(0.0, 0.0, 0.0), p3(10.0, 6.0, 4.0))?;
    let b = place(k, b)?;
    let bb = k.bounding_box(b)?;
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for x in [0.0, 10.0] { for y in [0.0, 6.0] { for z in [0.0, 4.0] {
        let q = point(p3(x, y, z));
        for i in 0..3 { lo[i] = lo[i].min(q[i]); hi[i] = hi[i].max(q[i]); }
    }}}
    let off = (0..3).map(|i| (lo[i] - bb.min[i]).abs().max((hi[i] - bb.max[i]).abs())).fold(0.0, f64::max);
    if off > 1e-6 { return Err(format!("placement mismatch: {off:e}").into()); }
    Ok(b)
}
