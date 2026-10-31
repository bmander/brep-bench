use super::bezier_dome::block;
use crate::solids::*;

/// The block under the dome of `nurbs/bezier-dome`, less the cylinder of radius 3 about the
/// line x = 12, y = 8, run from below the floor to above the dome.
pub fn build() -> Result<Body, crate::Refusal> {
    Ok(cut(&block()?, &zcyl(12.0, 8.0, 3.0, -1.0, 20.0)?)?)
}
