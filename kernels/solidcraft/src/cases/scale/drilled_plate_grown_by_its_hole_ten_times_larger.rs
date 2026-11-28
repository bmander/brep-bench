//! `offset/drilled-plate-grown-by-its-hole` ten times larger, built as it is at every length:
//! the plate from (−100, −100, 0) to (100, 100, 40) with a hole of radius 30 along the z axis,
//! grown by 30.
use crate::cases::offset::drilled_plate_grown_by_its_hole::grown;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    grown(10.0)
}
