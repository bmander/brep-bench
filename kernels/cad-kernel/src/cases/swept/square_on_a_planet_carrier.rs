//! The solid a square bar fills carried round on a planet: the prism from z = 0 to 2 on the square
//! of side 2 centred at (5, 0), its centre carried round the circle of radius 5 about z by t while
//! it turns about its own centre by −2t (by −3t against the carrier), t from 0° to 360°.
//!
//! Two turns about parallel axes at different rates are no constant twist, but every pose keeps the
//! plane z = 0, and the bar is a prism square to it: `swept::closed::closed` builds the region the
//! square sweeps in that plane — the envelopes of its sides and the trochoids its corners trace, each
//! stretch kept where it stands outside the square at every pose; the roll is a whole period, so no
//! end pose bounds it — fitted and carried from z = 0 to 2 (#880 Q2), and judges it by the sweep's
//! definition before returning it.
use crate::sketch::block;
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

pub fn build() -> Result<Brep, String> {
    let bar = block([4., -1., 0.], [6., 1., 2.])?;
    let up = [0., 0., 1.];
    // on the carrier the bar turns by −3t about its own centre; the carrier turns by t about z.
    // The motion is the planet's turn seen from a frame turning by −t about z (`Relative`: at each
    // t the source's pose, then the observer's undone), so the carrier's turn is put back.
    let planet = Trajectory::Twist(Screw {
        origin: [5., 0., 0.], axis: up, ratio: -3., advance: 0.,
    });
    let carrier = Trajectory::Twist(Screw {
        origin: [0., 0., 0.], axis: up, ratio: -1., advance: 0.,
    });
    let path = Trajectory::Relative { source: Box::new(planet), observer: Box::new(carrier) };
    // no constant twist, but a motion keeping the bar's plane: its section's planar sweep, extruded
    closed(&bar, &path, [0., TAU], bar.size(), tolerance(bar.size()))
        .map_err(|why| format!("the bar's sweep not built: {why:?}"))
}
