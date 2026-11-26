//! `singular/sectors-sharing-an-apex` ten times larger: two spherical sectors of the ball of radius
//! 100 about the origin, half-angle 45°, both with their apex at the origin: the first about +z,
//! the second the first turned 60° about the y axis. Each is the quarter disc's sector, the
//! triangle-and-arc from the axis to 45°, turned a whole turn about its axis. Their common part.
//! Built as the original is, every length times 10.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::solids::singular::turned;
use crate::Refusal;
use brep_kernel::{make_arc, make_line, revolve_profile_brep, BrepSolid};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let r = 100.0;
    let rim = p(r * FRAC_PI_4.sin(), 0.0, r * FRAC_PI_4.cos());
    // the arc in the xz plane from 45° off the axis up to the axis, angles from +x toward +z
    let section = [
        make_line(p(0.0, 0.0, 0.0), rim)?,
        make_arc(p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0), r, FRAC_PI_4, FRAC_PI_2)?,
        make_line(p(0.0, 0.0, r), p(0.0, 0.0, 0.0))?,
    ];
    let first = revolve_profile_brep(&section, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)?;
    let second = turned(&first, p(0.0, 0.0, 0.0), p(0.0, 1.0, 0.0), 60f64.to_radians(),
                        p(0.0, 0.0, 0.0))?;
    Ok(common(&first, &second)?)
}
