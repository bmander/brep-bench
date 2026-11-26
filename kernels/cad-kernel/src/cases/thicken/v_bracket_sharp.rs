//! A sheet of two flat faces creased at 60°: the floor, the rectangle from (0, 0, 0) to (10, 8, 0),
//! and the wall through (0, 0, 0), (0, 8, 0), (5, 8, 5√3), (5, 0, 5√3), leaning over it. Thickened
//! by 1 to the outside of its crease, the corner kept sharp.
use crate::sketch::{faces_through, polygon};
use cad_kernel::build::prism;
use cad_kernel::offset::{sheet_of, thickened, Join};
use cad_kernel::topo::Brep;

const WIDTH: f64 = 8.;

pub fn build() -> Result<Brep, String> {
    // the sheet's faces lifted off the solid they bound, their normals its outward ones: the
    // triangular prism over (0, 0), (10, 0), (5, 5√3) in (x, z), from y = 0 to 8, whose floor faces
    // down and whose wall over the crease faces away from the floor
    let h = 5. * 3f64.sqrt();
    let end = polygon(&[[0., 0., 0.], [5., 0., h], [10., 0., 0.]]);
    let solid = prism(&end, 0., WIDTH)?;
    let faces = faces_through(&solid, &[[5., WIDTH / 2., 0.], [2.5, WIDTH / 2., h / 2.]])?;
    thickened(&sheet_of(&solid, &faces)?, [0., 1.], Join::Sharp)
}
