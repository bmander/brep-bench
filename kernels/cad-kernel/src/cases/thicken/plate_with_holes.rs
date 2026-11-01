//! The square from (−10, −10, 0) to (10, 10, 0) with holes of radius 3 about (±5, 0, 0), a sheet,
//! thickened by 1 to each side.
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
    let plate = block([-10., -10., -1.], [10., 10., 0.])?;
    let mut solid = plate;
    for x in [-5., 5.] { solid = combined(&solid, &prism(&disc([x, 0., -2.], [0., 0., 1.], 3.), 0., 4.)?, Op::Cut, 0.)?; }
    let faces = faces_through(&solid, &[[0., 5., 0.]])?;
    thickened(&sheet_of(&solid, &faces)?, [-1., 1.], Join::Round)
}
