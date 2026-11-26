//! The plate from (−20, −15, 0) to (20, 15, 4) with a window cut through it, the rectangle from
//! (−10, −7) to (10, 7) with its corners rounded to radius 2, grown by 3, more than the corners'
//! radius, to its parallel body.
//!
//! The window's rounded rectangle is extruded from below the plate up past its top and cut away.
use crate::solids::offset::{parallel, subtract};
use crate::solids::outline::{deg, Chain};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, make_box_brep, transform_brep, AffineTransform, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let mut c = Chain::new();
    c.line((-8.0, -7.0), (8.0, -7.0))?
        .arc((8.0, -5.0), 2.0, deg(270.0), deg(360.0), true)?
        .line((10.0, -5.0), (10.0, 5.0))?
        .arc((8.0, 5.0), 2.0, deg(0.0), deg(90.0), true)?
        .line((8.0, 7.0), (-8.0, 7.0))?
        .arc((-8.0, 5.0), 2.0, deg(90.0), deg(180.0), true)?
        .line((-10.0, 5.0), (-10.0, -5.0))?
        .arc((-8.0, -5.0), 2.0, deg(180.0), deg(270.0), true)?;
    let window = extrude_profile_brep(&c.curves, p(0.0, 0.0, 1.0), 6.0)?;
    let down = AffineTransform { elements: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 1.0] };
    let window = transform_brep(&window, down, false)?;
    let plate = make_box_brep(p(-20.0, -15.0, 0.0), 40.0, 30.0, 4.0)?;
    let windowed = subtract(&plate, &window)?;
    Ok(parallel(&windowed, 3.0)?)
}
