//! The inner half of the torus about z of tube radius 2 about a middle circle of radius 6, a sheet,
//! thickened by 1 away from the middle circle, toward the axis.
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
    // the torus kept within the cylinder of radius 6 about z
    let solid = kept_by_solid(&torus([0., 0., 0.], [0., 0., 1.], 6., 2.)?, &Surface::Cylinder(Frame::WORLD, 6.), true, 0.)?;
    let faces = faces_through(&solid, &[[4., 0., 0.]])?;
    thickened(&sheet_of(&solid, &faces)?, [0., 1.], Join::Round)
}
