//! The solid a rod fills turned 120° about the z axis, which crosses it: the solid cylinder of
//! radius 0.5 about the x axis from x = −3 to 5.
//!
//! The kernel's closed sweep (`swept::closed::closed`) takes the rod whole. Its closed class
//! refuses it (`Unbuilt::Curved` at the centre of the rod's end at x = −3): under a turn a sharp
//! circle leaning to the axis is turned into its ring, and the rod's end rims, square to a radius,
//! have their normal in the plane through the turn's axis and their centre, which turns into no
//! ring the kernel reads (`turned::Rim::simple`); and the turn's axis crosses the rod's within its
//! side, where it would refuse the cylinder's contact too. Since 0.10.0 such a tool goes the
//! contact's road — each sheet on the surface of revolution of its section's meridian, cut where
//! the meridian crosses the axis, a pole of the sheets either side, and trimmed by membership in
//! the sweep.
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

const RADIUS: f64 = 0.5;
const TURN: f64 = 120f64.to_radians();
/// How closely the rod is meshed for the sweep's membership reads: a hundredth of its radius.
const BAR: f64 = 0.005;

pub fn build() -> Result<Brep, String> {
    let rod = prism(&disc([-3., 0., 0.], [1., 0., 0.], RADIUS), 0., 8.)?;
    // the turn about z: a screw that does not advance, turning a radian a radian of the roll
    let path = Trajectory::Twist(Screw {
        origin: [0., 0., 0.], axis: [0., 0., 1.], ratio: 1., advance: 0.,
    });
    // the kernel's closed swept solid: a cylinder about an axis square to the turn's, which the
    // turn's axis crosses within the face
    closed(&rod, &path, [0., TURN], BAR, tolerance(rod.size()))
        .map_err(|why| format!("the rod's sweep not built: {why:?}"))
}
