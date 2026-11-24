//! A block under a patch pinched to a point beside it, sliced by a plane past the pinch: the
//! bicubic Bézier patch (s³, t³, 1/2 + st), s = 2u − 1, t = 2v − 1, on the poles (xᵢ, xⱼ, 1/2 +
//! sᵢsⱼ), its parametrization singular at u = v = 1/2 alone; the block stands on z = 0 under its
//! part u, v ≥ 11/20 (1/1000 ≤ x, y ≤ 1), its top face on the whole patch, trimmed; kept where −x +
//! y + z/10 ≤ 13/250.
//!
//! Refused at run time by cad-kernel 0.23.0's Boolean: "a shell of the arrangement is kept on one
//! side of a bspline and dropped on another: two of its cells joined round an edge" (the meeting
//! with the plane is marched on past the face's rim through the patch's singular point, geomsolver
//! #1013). With the top face on the patch cut down to its part (`Net::segment`), the same face, it
//! builds, and measures the case's volume (and area) to 1e-10: 0.5244906492 (4.5919312120).
use crate::patches::{bezier, under};
use cad_kernel::build::kept;
use cad_kernel::topo::Brep;
use cad_kernel::util::space::unit;

const XS: [f64; 4] = [-1., 1., -1., 1.];
const SS: [f64; 4] = [-1., -1. / 3., 1. / 3., 1.];

pub fn build() -> Result<Brep, String> {
    let poles = (0..4)
        .map(|i| {
            (0..4)
                .map(|j| [XS[i], XS[j], 0.5 + SS[i] * SS[j]])
                .collect()
        })
        .collect();
    let a = under(&bezier(poles), [11. / 20., 1.], [11. / 20., 1.])?;
    // the side against the plane's normal (−1, 1, 1/10), through (1/2, 1/2 + 13/250, 0)
    kept(&a, [0.5, 0.5 + 13. / 250., 0.], unit([1., -1., -0.1]), 0.)
}
