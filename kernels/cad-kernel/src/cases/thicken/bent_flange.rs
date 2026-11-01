//! The profile from (−10, 0, 0) to the origin, round the quarter circle of radius 2 about (0, 0, 2)
//! to (2, 0, 2) and up to (2, 0, 12), swept 8 along y, a sheet, thickened by 1 away from the bend's
//! axis.
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
    // the bent profile, closed above and to the left, swept along y: its floor, bend and wall
    let bend = Frame::new([0., 0., 2.], [0., 1., 0.], [1., 0., 0.]);
    let section = Profile { origin: [0., 0., 0.], normal: [0., 1., 0.], names: Vec::new(), loops: vec![vec![
        ProfileEdge::Line { a: [-10., 0., 0.], b: [0., 0., 0.] },
        ProfileEdge::Arc { frame: bend, r: 2., span: Some([0., std::f64::consts::FRAC_PI_2]) },
        ProfileEdge::Line { a: [2., 0., 2.], b: [2., 0., 12.] },
        ProfileEdge::Line { a: [2., 0., 12.], b: [-10., 0., 12.] },
        ProfileEdge::Line { a: [-10., 0., 12.], b: [-10., 0., 0.] },
    ]] };
    let solid = prism(&section, 0., 8.)?;
    let faces = faces_through(&solid, &[[-5., 4., 0.], [1.4142135623730951, 4., 0.5857864376269049], [2., 4., 7.]])?;
    thickened(&sheet_of(&solid, &faces)?, [0., 1.], Join::Round)
}
