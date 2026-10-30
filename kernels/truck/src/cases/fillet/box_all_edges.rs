//! "A cube of side 10 with all twelve of its edges rounded to radius 1. Three rounds meet at each
//! corner."
//!
//! truck cannot build it. truck-shapeops rounds one edge at a time: `fillet_with_side` trims the
//! two faces either side of an edge and the two faces its ends run into, and wants each end to
//! meet exactly those three faces. Here every edge meets two other rounded edges at each corner,
//! and truck-shapeops' prototype for rounds along connected edges (`organize_characters`,
//! `fillet_faces`) is commented out in its source.
use crate::{Built, Refused};

pub fn build() -> Built {
    Err(Refused(
        "truck-shapeops rounds only a single edge whose ends each meet exactly three faces \
         (`fillet_with_side`); here three rounds meet at every corner, and its fillet along \
         connected edges is unsupported (commented out in truck-shapeops)"
            .into(),
    ))
}
