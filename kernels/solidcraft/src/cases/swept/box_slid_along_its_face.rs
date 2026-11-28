//! The solid the box from the origin to (10, 6, 4) fills as it slides along (8, 5, 0) without
//! turning: `swept/box-slid-aslant` slid level.
//!
//! Built as that case builds it (solidcraft-kernel has no sweep of a solid): the box where it
//! starts and the faces it leads with, x = 10 and y = 6, each slid the whole way. The top and
//! bottom slide within their own planes and lead nowhere.
use super::box_slid_aslant::slid;
use crate::solids::*;

pub fn build() -> R<Body> {
    slid(v3(8.0, 5.0, 0.0))
}
