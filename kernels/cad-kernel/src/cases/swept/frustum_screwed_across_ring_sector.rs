//! A ring sector (radii 8 to 12 about z, z from −1 to 1, polar angle 0° to 60°) with a helical
//! groove milled through it by a frustum about the vertical line through (10, 0), radius 0.4 at
//! z = −0.6 widening to 1 at z = 1.4, screwed about z — right-handed, rising 2 a turn — from a turn
//! of −140° to +140°.
//!
//! Refused by the kernel at run time. `cut_from` takes the frustum (a cone about an axis along the
//! screw's) and builds a solid, but its own judge (`swept::closed::judge`) finds it disagreeing
//! with the sweep's membership at a witness point inside the sweep and refuses it rather than
//! return it wrong (`Unbuilt::Wrong`). The same witness stands with the cutter meshed twice and
//! four times as finely and with the frustum built as its section revolved; the closed sweep built
//! alone (`swept::closed::closed`) is refused in its arrangement instead.
use crate::sketch::polygon;
use cad_kernel::build::{cone, revolve};
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

const SECTOR: f64 = 60f64.to_radians();
const LEAD: f64 = 2.;
const ROLL: [f64; 2] = [-140f64.to_radians(), 140f64.to_radians()];
/// How closely the cutter is meshed for the sweep's membership reads: a hundredth of its least
/// radius.
const BAR: f64 = 0.004;

/// The ring sector about z: radii 8 to 12, heights −1 to 1, polar angles 0° to 60° — its section in
/// the half-plane y = 0, x > 0 turned right-handed about z.
fn sector() -> Result<Brep, String> {
    let section = polygon(&[[8., 0., -1.], [12., 0., -1.], [12., 0., 1.], [8., 0., 1.]]);
    revolve(&section, [0., 0., 0.], [0., 0., 1.], SECTOR)
}

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let cutter = cone([10., 0., -0.6], up, 0.4, 1., 2.)?;
    // the screw about z: a radian a radian of the roll, rising the lead a turn
    let path = Trajectory::Twist(Screw {
        origin: [0., 0., 0.], axis: up, ratio: 1., advance: LEAD,
    });
    // the kernel's swept solid cut from the sector on one arrangement: a cone about an axis along
    // the screw's, touching along the two rulings where its normal is square to the screw's
    // velocity, its rims' discs screwed into ramps
    let blank = sector()?;
    let size = blank.size().max(cutter.size());
    cut_from(&blank, &cutter, &path, ROLL, BAR, tolerance(size))
        .map_err(|why| format!("the cutter's sweep not built: {why:?}"))
}
