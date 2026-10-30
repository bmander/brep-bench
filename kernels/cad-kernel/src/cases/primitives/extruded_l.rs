//! An L-shaped plate: (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) extruded 8 along +z.
use crate::sketch::polygon;
use cad_kernel::build::prism;
use cad_kernel::topo::Brep;

const THICKNESS: f64 = 8.;

pub fn build() -> Result<Brep, String> {
    let outline = polygon(&[
        [0., 0., 0.],
        [20., 0., 0.],
        [20., 5., 0.],
        [5., 5., 0.],
        [5., 15., 0.],
        [0., 15., 0.],
    ]);
    // counter-clockwise from above, so its normal is +z and the prism rises along it
    prism(&outline, 0., THICKNESS)
}
