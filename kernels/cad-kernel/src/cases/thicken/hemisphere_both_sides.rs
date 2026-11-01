//! The upper half of the sphere of radius 5 about the origin, a sheet, thickened by 1 to each side.
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
    // the upper half ball's sphere
    let solid = kept(&sphere([0., 0., 0.], [0., 0., 1.], 5.)?, [0., 0., 0.], [0., 0., 1.], 0.)?;
    let faces = faces_through(&solid, &[[3., 0., 4.]])?;
    thickened(&sheet_of(&solid, &faces)?, [-1., 1.], Join::Round)
}
