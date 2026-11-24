//! A block under a patch folded over its own rim, notched across the rim: the patch (s², v, 1/2 +
//! 3s⁴/10 + v/10), s = 2u − 1, the Bézier net of degree 4 by 1 on the poles below, folds back over
//! itself along u = 1/2; the block stands on z = 0 under its part u ≥ 13/20 (9/100 ≤ x ≤ 1), its
//! top face on the whole patch, trimmed; the box 1/20 ≤ x ≤ 1/2, −1 ≤ y ≤ 1/2, −1 ≤ z ≤ 3 is cut
//! from it.
//!
//! Refused at run time by cad-kernel 0.23.0's Boolean: "a bspline's pieces do not close" (the box's
//! side y = 1/2 meets the patch in one exact iso-curve over the whole net, and its stretch on the
//! folded-back layer is laid on the face a second time, geomsolver #1013). With the top face on the
//! patch cut down to its part (`Net::segment`), the same face, it builds, and measures the case's
//! volume (and area) to 1e-10: 0.48658855 (4.1267385567).
use crate::patches::{bezier, under};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::block;
use cad_kernel::topo::Brep;

const XS: [f64; 5] = [1., 0., -1. / 3., 0., 1.];
/// At v = 0; v = 1 a tenth higher.
const ZS: [f64; 5] = [0.8, 0.2, 0.8, 0.2, 0.8];

pub fn build() -> Result<Brep, String> {
    let patch = bezier(
        XS.iter()
            .zip(ZS)
            .map(|(&x, z)| vec![[x, 0., z], [x, 1., z + 0.1]])
            .collect(),
    );
    let a = under(&patch, [13. / 20., 1.], [0., 1.])?;
    let notch = block([1. / 20., -1., -1.], [0.5, 0.5, 3.])?;
    Ok(combined(&a, &notch, Op::Cut, 0.)?)
}
