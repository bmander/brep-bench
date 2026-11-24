//! A block under a patch whose folded-back layer lies just beneath it, pierced by a square hole:
//! the patch (s², v, 1/2 + 3s⁴/10 + s³/1000 + v/10), s = 2u − 1, the Bézier net of degree 4 by 1 on
//! the poles below, its layer u ≤ 1/2 standing x^{3/2}/500 below the face's; the block stands on
//! z = 0 under its part u ≥ 13/20 (9/100 ≤ x ≤ 1), its top face on the whole patch, trimmed; the
//! box 2/5 ≤ x ≤ 7/10, 3/10 ≤ y ≤ 3/5, −1 ≤ z ≤ 3 is cut through it.
//!
//! Refused at run time by cad-kernel 0.23.0's Boolean: "a shell of the arrangement is kept on one
//! side of a plane (`near`) and dropped on another: two of its cells joined round an edge"
//! (geomsolver #1013: the hole's upright edges read their crossings with the face and with the
//! patch's layer beneath it wrong, one pair as a touch, one face crossing lost; and its walls y =
//! 3/10 and y = 3/5, the patch's iso lines, are laid again along the layer beneath). With the top
//! face on the patch cut down to its part (`Net::segment`), the same face, it builds, and measures
//! the case's volume (and area) to 1e-10: 0.5433690755 (5.0702533214).
use crate::patches::{bezier, under};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::block;
use cad_kernel::topo::Brep;

const XS: [f64; 5] = [1., 0., -1. / 3., 0., 1.];
/// At v = 0; v = 1 a tenth higher.
const ZS: [f64; 5] = [0.799, 0.2005, 0.8, 0.1995, 0.801];

pub fn build() -> Result<Brep, String> {
    let patch = bezier(
        XS.iter()
            .zip(ZS)
            .map(|(&x, z)| vec![[x, 0., z], [x, 1., z + 0.1]])
            .collect(),
    );
    let a = under(&patch, [13. / 20., 1.], [0., 1.])?;
    let hole = block([0.4, 0.3, -1.], [0.7, 0.6, 3.])?;
    Ok(combined(&a, &hole, Op::Cut, 0.)?)
}
