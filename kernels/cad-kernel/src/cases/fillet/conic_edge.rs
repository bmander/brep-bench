//! The box from the origin to (20, 10, 10), the edge where its top z = 10 meets its end x = 20
//! blended by a conic of rho 0.5 (a parabola) tangent to the top and the end each 3 from the edge:
//! in every plane y = c the quadratic Bézier through (17, c, 10), (20, c, 10) and (20, c, 7).
//!
//! Refused: cad-kernel 0.9.0 blends an edge only by a ball (`fillet::roll`, a circular section)
//! or a flat (`fillet::flat_along`, a chamfer); it has no blend whose section is a conic. The block
//! is built and the edge picked as `roll` would pick it; the missing operation is
//!
//! ```text
//! fillet::conic_along(b: &Brep, at: V, legs: [f64; 2], rho: f64, tol: f64)
//!     -> Result<Rolled, String>
//! ```
//!
//! the run of edges nearest `at` blended by the conic of `rho` tangent to each face `legs[k]` from
//! the edge, its piece taken away (convex) or added (concave) by `boolean::combined` as a rolled
//! round's is. Cutting the block by a prism of the parabola instead would place the blend by hand.
use crate::sketch::block;
use cad_kernel::topo::Brep;

const LENGTH: f64 = 20.;
const WIDTH: f64 = 10.;
const HEIGHT: f64 = 10.;
const LEGS: [f64; 2] = [3., 3.];
const RHO: f64 = 0.5;

pub fn build() -> Result<Brep, String> {
    let solid = block([0., 0., 0.], [LENGTH, WIDTH, HEIGHT])?;
    let at = [LENGTH, WIDTH / 2., HEIGHT];
    Err(format!("no conic blend: cad-kernel 0.9.0 rounds an edge by a ball (fillet::roll) or cuts \
                 it flat (fillet::flat_along), and the edge at {at:?} of this {}-face block wants \
                 fillet::conic_along(b, at, legs: {LEGS:?}, rho: {RHO}, tol) -> Result<Rolled, \
                 String>", solid.faces.len()))
}
