//! Five faces of the cube from the origin to (10, 10, 10), all but its top, a sheet, thickened by 1
//! into the cube.
#[allow(unused_imports)]
use crate::sketch::{block, polygon, faces_through, ball_below};
#[allow(unused_imports)]
use cad_kernel::boolean::{combined, Op};
#[allow(unused_imports)]
use cad_kernel::build::{cone, disc, kept, kept_by_solid, prism, sphere, torus, Profile, ProfileEdge};
#[allow(unused_imports)]
use cad_kernel::geom::{Frame, Surface};
use cad_kernel::offset::{sheet_of, thickened, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the open box: the cube's faces but its top, their normals out of the cube
    let solid = block([0., 0., 0.], [10., 10., 10.])?;
    let faces = faces_through(&solid, &[[5., 5., 0.], [0., 5., 5.], [10., 5., 5.], [5., 0., 5.], [5., 10., 5.]])?;
    thickened(&sheet_of(&solid, &faces)?, [-1., 0.], Join::Round)
}
