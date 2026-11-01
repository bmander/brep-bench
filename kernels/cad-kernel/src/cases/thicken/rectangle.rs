//! The rectangle from (0, 0, 0) to (10, 8, 0), a sheet, thickened by 2 to its +z side.
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
    // the sheet: the top of a block, its normal +z
    let solid = block([0., 0., -1.], [10., 8., 0.])?;
    let faces = faces_through(&solid, &[[5., 4., 0.]])?;
    thickened(&sheet_of(&solid, &faces)?, [0., 2.], Join::Round)
}
