//! `sweep/helical-pipe` forty turns long, built as it is: the pipe of radius 1 along the helix of
//! radius 10 rising 5 a turn, from (10, 0, 0) to (10, 0, 200).
use super::helical_pipe::coil;
use crate::solids::*;

pub fn build() -> R<Body> {
    coil(40.0)
}
