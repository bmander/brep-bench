//! A worm wheel hobbed by its worm.
//!
//! The kernel's sweep of a screw thread under two turns (`solid_sweep_thread`), the wheel cut
//! across its rim: the worm's thread (axis through (0, 40, 0) along x, lead 2π, pitch radius 10,
//! π wide there, flanks 20°, radii 7.5 to 12.5, ends x = ±20), turned by t about its axis and then
//! by −t/30 about the z axis for t from 0 to 60π, cut from the disk of radius 32 between z = −6 and
//! z = 6. The kernel traces one tooth space on planes square to the wheel's axis, cuts it from the
//! blank's wedge between two teeth and sews the 30 wedges: the sweep, its spaces and the wheel are
//! all made inside it, from the case's data alone.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::solid_sweep_thread::{thread_wheel, RingBlank, ThreadTool};
use brep_kernel::{BrepSolid, TwoTurns};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let axis = (p(0.0, 40.0, 0.0), p(1.0, 0.0, 0.0));
    let thread = ThreadTool {
        axis_point: axis.0,
        axis_direction: axis.1,
        // θ = 0 along +y, right-handed about +x.
        reference: p(0.0, 1.0, 0.0),
        lead: TAU,
        pitch_radius: 10.0,
        taper: 0.0,
        half_width: FRAC_PI_2,
        flank_tan: 20f64.to_radians().tan(),
        tip: 2.5,
        root: 2.5,
        start: -20.0,
        end: 20.0,
    };
    let motion = TwoTurns { axes: [axis, (p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0))], rates: [1.0, -1.0 / 30.0], from: 0.0, to: 60.0 * PI };
    let blank = RingBlank { inner: 0.0, outer: 32.0, bottom: -6.0, face: 6.0, face_outer: 6.0 };
    Ok(thread_wheel(&thread, &motion, &blank, 30)?)
}
