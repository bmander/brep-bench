//! The torus and plane of `tangent/villarceau-half-torus`, ten thousand times larger: a torus
//! about z of radii 50000 and 30000, kept on and below the plane 3x − 4z = 0, which touches it at
//! two points and cuts it in Villarceau's circles.
use cad_kernel::build::{kept, revolve, Profile, ProfileEdge};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const MAJOR: f64 = 50000.;
const MINOR: f64 = 30000.;

pub fn build() -> Result<Brep, String> {
    // the tube's section in the plane x = 0 on the side y > 0, turned about the z axis: the seam
    // where it starts is there, away from the tangent points, as the case asks
    let axis = Frame::new([0., 0., 0.], [0., 0., 1.], [0., 1., 0.]);
    let centre = axis.at([MAJOR, 0., 0.]);
    let section = ProfileEdge::Arc { frame: Frame::new(centre, axis.y, axis.x), r: MINOR, span: None };
    let tube = Profile { origin: centre, normal: axis.y, loops: vec![vec![section]], names: Vec::new() };
    let torus = revolve(&tube, axis.o, axis.z, TAU)?;
    // the side of the plane its unit normal (3, 0, −4)/5 points to, where z ≤ 3x/4
    kept(&torus, [0., 0., 0.], [0.6, 0., -0.8], 0.)
}
