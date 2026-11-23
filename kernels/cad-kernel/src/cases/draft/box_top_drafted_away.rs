//! The box from the origin to (30, 10, 10), its four sides drafted 30° inward about z = 0, which
//! consumes its top: a hip roof, its ridge at 5√3.
use crate::sketch::{block, drafted_face};
use cad_kernel::topo::Brep;

const SIZE: [f64; 3] = [30., 10., 10.];
const DRAFT: f64 = 30.;

pub fn build() -> Result<Brep, String> {
    // cad-kernel drafts a profile as it extrudes it (`build::drafted`), but refuses a section the
    // draft takes to nothing below the top ("a draft takes a side to nothing 8.6603 from the
    // parting plane"), and it has no draft of a solid's faces. A drafted side is the plane of the
    // side turned about its bottom edge, so the drafted box is the box kept inside each of the four
    // turned planes (`drafted_face`), the two long ones closing in the ridge under the top
    let draft = DRAFT.to_radians();
    let [a, b, _] = SIZE;
    let mut solid = block([0., 0., 0.], SIZE)?;
    for (on, outward) in [([0., 0., 0.], [0., -1., 0.]), ([a, 0., 0.], [1., 0., 0.]),
                          ([a, b, 0.], [0., 1., 0.]), ([0., b, 0.], [-1., 0., 0.])] {
        solid = drafted_face(&solid, on, outward, draft)?;
    }
    Ok(solid)
}
