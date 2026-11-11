//! A wheel, the disc of radius 40 about the vertical line through (−50, 0) from z = −8 to −2, cut
//! by a plus (arms 30 long and 8 wide along ±x and ±y, extruded from z = −10 to 0) turning about z
//! by t while the wheel turns by −2t/3, over t from 0° to 540°: what is left is what lies outside
//! the tool at every instant.
//!
//! The kernel's one sweep operation builds the plus's swept solid alone, as the wheel sees it, and
//! the ordinary Boolean cuts it from the wheel. Seen from the wheel the plus rolls in its own plane,
//! a prism square to it, so `swept::closed::closed` builds the region the plus sweeps in the plane —
//! the envelopes of its sides and the trochoids of its corners, each stretch kept where it stands
//! outside the plus at every pose; the roll is a whole period of the pair, so no end pose bounds it —
//! fitted and carried through the plus's depth (#880 Q2), and judges it by the sweep's definition
//! before returning it.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::PI;

/// How closely the plus is meshed for the sweep's membership reads (its faces are flat).
const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let wheel = prism(&disc([-50., 0., -8.], up, 40.), 0., 6.)?;
    let (a, w) = (30., 4.);
    let corners = [[a, -w], [a, w], [w, w], [w, a], [-w, a], [-w, w], [-a, w], [-a, -w], [-w, -w], [-w, -a], [w, -a], [w, -w]];
    let plus = prism(&polygon(&corners.map(|[x, y]| [x, y, -10.])), 0., 10.)?;
    // the plus turning by t about z, seen from the wheel turning by −2t/3 about its own axis
    let tool_turn = Trajectory::Twist(Screw { origin: [0.; 3], axis: up, ratio: 1., advance: 0. });
    let wheel_turn = Trajectory::Twist(Screw { origin: [-50., 0., 0.], axis: up, ratio: -2. / 3., advance: 0. });
    let path = Trajectory::Relative { source: Box::new(tool_turn), observer: Box::new(wheel_turn) };
    let swept = closed(&plus, &path, [0., 3. * PI], BAR, tolerance(plus.size()))
        .map_err(|why| format!("the plus's sweep not built: {why:?}"))?;
    Ok(combined(&wheel, &swept, Op::Cut, 0.)?)
}
