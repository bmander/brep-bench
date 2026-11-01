//! The whole cylinder of radius 5 about z from z = 0 to 10, a sheet closing round on itself,
//! thickened by 1 away from the axis.
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
    let solid = prism(&disc([0., 0., 0.], [0., 0., 1.], 5.), 0., 10.)?;
    let faces = faces_through(&solid, &[[0., 5., 5.]])?;
    thickened(&sheet_of(&solid, &faces)?, [0., 1.], Join::Round)
}
