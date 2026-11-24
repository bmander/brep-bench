//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a leaning bar
//! passes through as it turns about the z axis from −140° to 140°: the box whose inner side is the
//! plane through (10, 0, 0) square to u = (cos 20°, 0, −sin 20°), 3 deep along u, within 2 of the
//! plane y = 0 and within 3 of (10, 0, 0) along l = (sin 20°, 0, cos 20°).
//!
//! springrep's sweep of a solid declines this one (faces round an edge do not pair off). Within the
//! block's reach (12 from the z axis) every circle about the z axis meets the bar in one arc across
//! its middle plane y = 0, so there the sweep is the bar's section by that plane, a rectangle
//! leaning 20°, turned through the 280° (`shared::swept`, a turn); the bar is clear of the block
//! where it starts and stops.
use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, polygon, sector_block, swing};

pub fn build() -> Result<Brep, String> {
    let (s, c) = deg(20.0).sin_cos();
    // corners in (x, z): (10, 0) ∓ 3l, then 3 along u from each
    let p0 = (10.0 - 3.0 * s, -3.0 * c);
    let p1 = (10.0 + 3.0 * s, 3.0 * c);
    let p2 = (p1.0 + 3.0 * c, p1.1 - 3.0 * s);
    let p3 = (p0.0 + 3.0 * c, p0.1 - 3.0 * s);
    let section = polygon(&[p0, p3, p2, p1], "bar");
    let (from, to) = (deg(-140.0), deg(140.0));
    let cut = swing(&section, from, to - from, "cut")?;
    difference(&sector_block()?, &cut).map_err(e)
}
