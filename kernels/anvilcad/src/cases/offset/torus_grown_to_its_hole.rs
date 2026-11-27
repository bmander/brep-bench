//! A torus about the z axis (middle circle 4, tube 2) grown by 2, to its hole: its one face is the
//! tube of radius 4, which just closes over the axis, a horn torus.
use anvil_brep::{Frame, Solid};

use crate::shared::{offset, torus};

pub fn build() -> Result<Solid, String> {
    // the torus has no edge to round: growing it is moving its one face
    offset(&torus(&Frame::world(), 4., 2.)?, 2.)
}
