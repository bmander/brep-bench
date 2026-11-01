//! The cap of the sphere of radius 10 about the origin where z ≥ 6, a sheet, thickened by 2 away
//! from the centre.
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
    let solid = kept(&sphere([0., 0., 0.], [0., 0., 1.], 10.)?, [0., 0., 6.], [0., 0., 1.], 0.)?;
    let faces = faces_through(&solid, &[[0., 6., 8.]])?;
    thickened(&sheet_of(&solid, &faces)?, [0., 2.], Join::Round)
}
