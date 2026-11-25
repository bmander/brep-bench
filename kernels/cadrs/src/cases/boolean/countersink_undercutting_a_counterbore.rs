//! A disc (radius 100, z from −20 to 20) counterbored (radius 60 from z = 0 up), with a hole of
//! radius 3.5 about the upright through (59.9, 0) from z = −15 to 0 and its countersink (the 90°
//! cone through the circle of radius 4 on z = 0, its point at z = −4), both reaching under the
//! counterbore's wall.
//!
//! The disc, the counterbore and the hole are circles extruded and the countersink cadrs's cone
//! primitive; the three cut from the disc one after another.
use crate::bench::{cone, cut_each, cylinder, p3, v3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let up = v3(0.0, 0.0, 1.0);
    let disc = cylinder(k, p3(0.0, 0.0, -20.0), up, 100.0, 40.0)?;
    let counterbore = cylinder(k, p3(0.0, 0.0, 0.0), up, 60.0, 30.0)?;
    let hole = cylinder(k, p3(59.9, 0.0, -15.0), up, 3.5, 15.0)?;
    let countersink = cone(k, p3(59.9, 0.0, -4.0), up, 0.0, 4.0, 4.0)?;
    cut_each(k, disc, &[counterbore, hole, countersink])
}
