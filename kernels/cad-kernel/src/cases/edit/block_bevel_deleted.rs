//! The box from the origin to (20, 10, 10), the edge where its top, z = 10, meets its side x = 20
//! beveled 3 back on each face, the bevel the plane through the lines x = 17, z = 10 and x = 20,
//! z = 7; the bevel then deleted, the top running on from x = 17 and the side from z = 7 until they
//! meet in the edge x = 20, z = 10.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_through, polygon};
use cad_kernel::build::prism;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the pentagon walked clockwise in (x, z), so it faces +y, swept 10 along y
    let section = polygon(&[[0., 0., 0.], [0., 0., 10.], [17., 0., 10.], [20., 0., 7.], [20., 0., 0.]]);
    let beveled = prism(&section, 0., 10.)?;
    let bevel = faces_through(&beveled, &[[18.5, 5., 8.5]])?;
    deleting(&beveled, &bevel)
}
