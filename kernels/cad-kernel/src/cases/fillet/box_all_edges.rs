//! A cube of side 10 with all twelve edges rounded to radius 1, three rounds meeting at each corner.
//!
//! Refused. What cad-kernel offers, and why none of it builds this:
//!
//! - `fillet::roll(&cube, at, r, tol)` rounds the run of edges nearest `at` between two faces, the
//!   kernel placing the ball and building the piece it takes off (it rounds `box-one-edge` and
//!   `cylinder-rim`). On this cube it rolls the four vertical edges, but refuses each top and
//!   bottom edge: against the plain cube, "two canals meeting in a mitre are rung 3"; after the
//!   vertical rounds, "the ball … is larger than `near` can hold" (its band reaches the edge a
//!   vertical round left on the top, and that round is a fitted B-spline face no ball rolls on).
//! - The ball's patch where three rounds meet is `fillet::corner_among(vertex, centre, r, faces)`,
//!   or a recipe `lowered` node's `corners`, and both take the ball's centre from the caller, as a
//!   lowered node takes its runs (prisms and revolutions) and the sections its rolled runs are cut
//!   at. Building those here would be the caller placing the rounds, not the kernel.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no rounding of edges that meet at a vertex that the kernel places: fillet::roll rounds the \
         cube's four vertical edges but refuses its top and bottom ones (\"two canals meeting in a mitre \
         are rung 3\"; once the vertical edges are rounded, \"larger than `near` can hold\"), and the \
         ball's patch where three rounds meet (fillet::corner_among, a recipe's lowered corner) takes \
         its centre from the caller"
        .into())
}
