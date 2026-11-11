//! The solid a regular tetrahedron fills screwed about the z axis — right-handed, rising 5 a turn
//! — from a turn of 0° to 540°: the tetrahedron on (5, 1, 1), (5, −1, −1), (3, 1, −1), (3, −1, 1).
use cad_kernel::build::hull;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

const CORNERS: [V; 4] = [[5., 1., 1.], [5., -1., -1.], [3., 1., -1.], [3., -1., 1.]];
const LEAD: f64 = 5.;
const TURN: f64 = 540f64.to_radians();

pub fn build() -> Result<Brep, String> {
    // the tetrahedron: its corners' convex hull
    let tool = hull(&CORNERS)?;
    // the screw about z: a radian a radian of the roll, rising the lead a turn
    let path = Trajectory::Twist(Screw {
        origin: [0., 0., 0.], axis: [0., 0., 1.], ratio: 1., advance: LEAD,
    });
    // the kernel's closed swept solid: a polyhedral tool whose edges lean to the screw's axis,
    // each screwed into a patch of the ruled helicoid (its flat faces mesh exactly, at any bar)
    closed(&tool, &path, [0., TURN], tool.size(), tolerance(tool.size()))
        .map_err(|why| format!("the tetrahedron's sweep not built: {why:?}"))
}
