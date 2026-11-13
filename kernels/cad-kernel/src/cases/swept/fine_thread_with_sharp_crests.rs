//! A rod, the cylinder of radius 5 about z from z = 0 to 20, threaded by the bar 4.5 ≤ x ≤ 12,
//! −8 ≤ y ≤ 8, −0.45 ≤ z ≤ 0.45 screwed about z, right-handed and rising 1 a turn, from a turn of
//! −180° to 7380°: the groove's turns meet, leaving a thread with sharp crests.
//!
//! Built by the kernel's closed sweep cut from the rod: `cut_from` takes the bar (its edges run
//! along the screw's axis or square to it) and meets them with its helicoidal sheets, which the
//! lead of 1 packs 21 turns of, a hair apart at the sharp crests.
use crate::sketch::block;
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
const LEAD: f64 = 1.;
const ROLL: [f64; 2] = [-180f64.to_radians(), 7380f64.to_radians()];

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0., 0., 0.], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    let bar = block([4.5, -8., -0.45], [12., 8., 0.45])?;
    // the screw about z: a radian a radian of the roll, rising the lead a turn
    let path = Trajectory::Twist(Screw {
        origin: [0., 0., 0.], axis: up, ratio: 1., advance: LEAD,
    });
    // the kernel's swept solid cut from the rod on one arrangement: the bar's edges along the axis
    // and square to it, screwed into cylinders and helicoidal sheets (a box's flat faces mesh
    // exactly, at any bar)
    let size = rod.size().max(bar.size());
    cut_from(&rod, &bar, &path, ROLL, bar.size(), tolerance(size))
        .map_err(|why| format!("the bar's sweep not built: {why:?}"))
}
