//! The bored frustum of `shell/bored-frustum-sealed-below-its-opening` with every length 3.1 times
//! as long: the disk of radius 18.6 at z = 0 up to the disk of radius 9.3 at z = 12.4, bored through
//! by a hole of radius 3.1, shelled inward to a wall 3.1 thick, open at its top. The moved side and
//! the moved bore close on one another below the top, so the cavity is a closed ring and the top
//! stays whole.
use crate::sketch::faces_through;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, disc, prism};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

/// How many times the lengths of `shell/bored-frustum-sealed-below-its-opening`.
const SCALE: f64 = 3.1;
const BASE_RADIUS: f64 = 6. * SCALE;
const TOP_RADIUS: f64 = 3. * SCALE;
const HEIGHT: f64 = 4. * SCALE;
const BORE_RADIUS: f64 = 1. * SCALE;
const THICKNESS: f64 = 1. * SCALE;

pub fn build() -> Result<Brep, String> {
    let frustum = cone([0., 0., 0.], [0., 0., 1.], BASE_RADIUS, TOP_RADIUS, HEIGHT)?;
    // the drill runs on past both ends, so its ends cut nothing
    let drill = prism(&disc([0., 0., -SCALE], [0., 0., 1.], BORE_RADIUS), 0., HEIGHT + 2. * SCALE)?;
    let bored = combined(&frustum, &drill, Op::Cut, 0.)?;
    // the top, the annulus between the bore and the rim, picked by a point on it
    let open = faces_through(&bored, &[[0., (BORE_RADIUS + TOP_RADIUS) / 2., HEIGHT]])?;
    shelled(&bored, &open, THICKNESS, Side::In, Join::Round)
}
