//! An L-block, (0, 0), (10, 0), (10, 3), (3, 3), (3, 10), (0, 10) in xz extruded along y from 0 to
//! 10, its inside edge x = 3, z = 3 chamfered 7 on both faces, which consumes both faces beside it.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::fillet::flat_along;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const LENGTH: f64 = 10.;
const INSIDE: f64 = 3.;
const DISTANCE: f64 = 7.;

pub fn build() -> Result<Brep, String> {
    let mut section = polygon(&[
        [0., 0., 0.],
        [10., 0., 0.],
        [10., 0., INSIDE],
        [INSIDE, 0., INSIDE],
        [INSIDE, 0., 10.],
        [0., 0., 10.],
    ]);
    // each side named by its edge of the section, the two inside ones `floor` (z = 3) and `wall`
    // (x = 3)
    let names = ["bottom", "end", "floor", "wall", "top", "back"];
    section.names = vec![names.map(String::from).to_vec()];
    // counter-clockwise in (x, z), so its normal is −y: ordinates −10 to 0 along it are y 10 to 0
    let block = prism(&section, -LENGTH, 0.)?;
    // the inside edge is picked by a point on it; the kernel cuts the flat between the face z = 3
    // at x = 10 and the face x = 3 at z = 10, and builds the prism between it and the edge
    let legs = [DISTANCE, DISTANCE];
    let at = [INSIDE, LENGTH / 2., INSIDE];
    let piece = flat_along(&block, at, legs, None, tolerance(block.size()))?;
    // the edge is concave: its piece is filled in
    Ok(combined(&block, &piece, Op::Union, 0.)?)
}
