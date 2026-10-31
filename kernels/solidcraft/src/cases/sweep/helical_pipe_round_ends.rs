use super::helical_pipe::{PIPE, PITCH, RADIUS, TURNS, pipe};
use crate::solids::*;
use solidcraft_kernel::sphere;

/// The flat-ended helical pipe with a ball of the pipe's radius about each end of the helix:
/// the half of each ball behind its end lies inside the pipe, so the union is the pipe closed
/// by a half ball at each end, as a ball rolled along the helix leaves. (The kernel declines
/// the union, where the ball touches the pipe's wall all round the end, after a minute and a
/// half; a union with a half ball, cut from the ball by the end's plane or revolved from a
/// quarter disc, runs past five minutes.)
pub fn build() -> R<Body> {
    let start = v3(RADIUS, 0.0, 0.0);
    let end = v3(RADIUS, 0.0, PITCH * TURNS);
    let pipe = union(&pipe()?, &sphere(start, PIPE)?)?;
    union(&pipe, &sphere(end, PIPE)?)
}
