//! An open tray on its back, opening toward −y: the block from (−36, 0, −24) to (36, 16, 24) less
//! the pocket from (−33, 0, −21) to (33, 13, 21), and four tubes along y joined to its floor, outer
//! radius 5 and inner 2, from y = 5 to y = 13, on the axes through (x, z) = (±26, ±14).
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const BLOCK: ([f64; 3], [f64; 3]) = ([-36.0, 0.0, -24.0], [36.0, 16.0, 24.0]);
const POCKET: ([f64; 3], [f64; 3]) = ([-33.0, 0.0, -21.0], [33.0, 13.0, 21.0]);
const OUTER: f64 = 5.0;
const INNER: f64 = 2.0;
const STANDOFF_FROM: f64 = 5.0;
const STANDOFF_TO: f64 = 13.0;
const AXES: [(f64, f64); 4] = [(26.0, 14.0), (-26.0, 14.0), (-26.0, -14.0), (26.0, -14.0)];

fn block((low, high): ([f64; 3], [f64; 3])) -> Solid {
    Solid::cube(high[0] - low[0], high[1] - low[1], high[2] - low[2])
        .translate(low[0], low[1], low[2])
}

/// A cylinder of `radius` along +y from `y0` to `y1`, on the axis through (x, z).
fn along_y(radius: f64, (x, z): (f64, f64), y0: f64, y1: f64) -> Solid {
    // a cylinder along z turned −90° about x lies along +y
    Solid::cylinder(radius, y1 - y0, SEGMENTS).rotate(-90.0, 0.0, 0.0).translate(x, y0, z)
}

pub fn build() -> Result<Solid, crate::Refusal> {
    // the pocket opens through the face y = 0: its box runs 1 past that face
    let (low, high) = POCKET;
    let pocket = block(([low[0], low[1] - 1.0, low[2]], high));
    let mut tray = block(BLOCK).try_difference(&pocket)?;
    for axis in AXES {
        // the tube's hole is cut through the tube, running 1 past both its ends; the floor it
        // stands on closes the hole's far end once the tube is joined
        let core = along_y(INNER, axis, STANDOFF_FROM - 1.0, STANDOFF_TO + 1.0);
        let tube = along_y(OUTER, axis, STANDOFF_FROM, STANDOFF_TO).try_difference(&core)?;
        tray = tray.try_union(&tube)?;
    }
    Ok(tray)
}
