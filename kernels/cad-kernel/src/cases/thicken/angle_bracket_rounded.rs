//! A floor (z = 0, 10 by 8) and a wall (x = 0, 8 by 10) meeting at a right angle along y, a sheet,
//! thickened by 1 to the outside of the crease (−z and −x), its corner rounded.
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
    // the sheet's faces lifted off the solid they bound, their normals its outward ones
    let solid = block([0., 0., 0.], [10., 8., 10.])?;
    let faces = faces_through(&solid, &[[5., 4., 0.], [0., 4., 5.]])?;
    thickened(&sheet_of(&solid, &faces)?, [0., 1.], Join::Round)
}
