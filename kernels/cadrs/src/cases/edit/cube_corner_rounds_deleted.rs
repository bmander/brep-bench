//! The box from the origin to (10, 10, 10), the three edges meeting at (10, 10, 10) rounded to
//! radius 2, an eighth of a ball where they meet; then the three rounds and the ball deleted, the
//! faces x = 10, y = 10 and z = 10 run on to meet in the edges and the corner again: the cube.
//!
//! The cube is a square extruded and the three edges filleted together, radius 2; then Delete face
//! takes the three rounds and the ball.
use crate::bench::{block, p3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let cube = block(k, p3(0.0, 0.0, 0.0), p3(10.0, 10.0, 10.0))?;
    let edges = k.edges_at(cube, &[p3(5.0, 10.0, 10.0), p3(10.0, 5.0, 10.0), p3(10.0, 10.0, 5.0)])?;
    let rounded = fillet(k, cube, &edges, 2.0)?;
    // each round at 45° about its axis, and the ball along the diagonal from its center (8, 8, 8)
    let (s, t) = (8.0 + 2f64.sqrt(), 8.0 + 2.0 / 3f64.sqrt());
    let blends = k.faces_at(rounded, &[p3(4.0, s, s), p3(s, 4.0, s), p3(s, s, 4.0), p3(t, t, t)])?;
    deleted(k, rounded, &blends)
}
