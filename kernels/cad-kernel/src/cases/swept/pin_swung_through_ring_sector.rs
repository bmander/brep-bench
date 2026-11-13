//! A ring sector (radii 8 to 12 about z, z from −1 to 1, polar angle 0° to 60°) with a slot swung
//! through it by a pin, the solid cylinder of radius 0.5 about the x axis from x = 6 to 11, turned
//! rigidly about z from −140° to +140°.
//!
//! The kernel's closed sweep (`swept::closed::cut_from`) takes the pin whole. Its closed class
//! refuses it (`Unbuilt::Curved` at the centre of the pin's near end): under a turn a sharp circle
//! leaning to the axis is turned into its ring, and the pin's end rims, square to a radius, have
//! their normal in the plane through the turn's axis and their centre, which turns into no ring
//! the kernel reads (`turned::Rim::simple`). Since 0.10.0 such a tool goes the contact's road —
//! each sheet on the surface of revolution of its section's meridian, its rails circles about the
//! axis, trimmed by membership in the sweep, and the result cut from the sector.
use crate::sketch::polygon;
use cad_kernel::build::{disc, prism, revolve};
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

const SECTOR: f64 = 60f64.to_radians();
const ROLL: [f64; 2] = [-140f64.to_radians(), 140f64.to_radians()];
/// How closely the pin is meshed for the sweep's membership reads: a hundredth of its radius.
const BAR: f64 = 0.005;

/// The ring sector about z: radii 8 to 12, heights −1 to 1, polar angles 0° to 60° — its section in
/// the half-plane y = 0, x > 0 turned right-handed about z.
fn sector() -> Result<Brep, String> {
    let section = polygon(&[[8., 0., -1.], [12., 0., -1.], [12., 0., 1.], [8., 0., 1.]]);
    revolve(&section, [0., 0., 0.], [0., 0., 1.], SECTOR)
}

pub fn build() -> Result<Brep, String> {
    let pin = prism(&disc([6., 0., 0.], [1., 0., 0.], 0.5), 0., 5.)?;
    // the turn about z: a screw that does not advance, turning a radian a radian of the roll
    let path = Trajectory::Twist(Screw {
        origin: [0., 0., 0.], axis: [0., 0., 1.], ratio: 1., advance: 0.,
    });
    // the kernel's swept solid cut from the sector on one arrangement: a cylinder about an axis
    // square to the turn's, its contact the two generators in the plane of both axes, and its far
    // end's rim, a sharp circle leaning to the axis, turned into its ring
    let blank = sector()?;
    let size = blank.size().max(pin.size());
    cut_from(&blank, &pin, &path, ROLL, BAR, tolerance(size))
        .map_err(|why| format!("the pin's sweep not built: {why:?}"))
}
