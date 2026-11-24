//! A block under a cuspidal patch, sliced by a plane through the cusp beside it: the patch (s², v,
//! 1/2 + s³ + v/10), s = 2u − 1, the Bézier net of degree 3 by 1 on the poles below, its two layers
//! meeting in a cusp along u = 1/2; the block stands on z = 0 under its part u ≥ 11/20 (1/100 ≤ x ≤
//! 1), its top face on the whole patch, trimmed; kept on or below the plane z = 11y/10.
//!
//! Refused at run time by cad-kernel 0.23.0's Boolean: "a shell of the arrangement is kept on one
//! side of a bspline and dropped on another: two of its cells joined round an edge" (the meeting
//! with the plane is marched on past the face's rim into the cusp and back, geomsolver #1013). With
//! the top face on the patch cut down to its part (`Net::segment`), the same face, it builds, and
//! measures the case's volume (and area) to 1e-10: 0.5103127217 (4.2949491491).
use crate::patches::{bezier, under};
use cad_kernel::build::kept;
use cad_kernel::topo::Brep;
use cad_kernel::util::space::unit;

const XS: [f64; 4] = [1., -1. / 3., -1. / 3., 1.];
/// At v = 0; v = 1 a tenth higher.
const ZS: [f64; 4] = [-0.5, 1.5, -0.5, 1.5];

pub fn build() -> Result<Brep, String> {
    let patch = bezier(
        XS.iter()
            .zip(ZS)
            .map(|(&x, z)| vec![[x, 0., z], [x, 1., z + 0.1]])
            .collect(),
    );
    let a = under(&patch, [11. / 20., 1.], [0., 1.])?;
    // below the plane: against its upward normal (0, −11/10, 1)
    kept(&a, [0.5, 0.5, 0.55], unit([0., 1.1, -1.]), 0.)
}
