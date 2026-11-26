//! The five-pointed star prism, the outline through ten points alternately 10 and 5 from the
//! origin, every 36° from (0, 10), extruded 2 along +z, grown by 1/2 to its parallel body: its
//! convex edges rounded, its five concave vertical edges left sharp.
use crate::solids::offset::parallel;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, BrepSolid};
use std::f64::consts::{FRAC_PI_2, PI};

pub fn build() -> Result<BrepSolid, Refusal> {
    let corners: Vec<_> = (0..10)
        .map(|i| {
            let angle = FRAC_PI_2 + i as f64 * PI / 5.0;
            let rho = if i % 2 == 0 { 10.0 } else { 5.0 };
            p(rho * angle.cos(), rho * angle.sin(), 0.0)
        })
        .collect();
    let star = extrude_profile_brep(&polygon(&corners)?, p(0.0, 0.0, 1.0), 2.0)?;
    Ok(parallel(&star, 0.5)?)
}
