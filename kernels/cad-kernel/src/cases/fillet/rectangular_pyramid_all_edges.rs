//! The pyramid on the rectangle from (−10, −6, 0) to (10, 6, 0), its apex at (0, 0, 12), all
//! eight edges rounded to radius 3/2: a ball's patch at each corner of the base, and the apex split
//! into a short round between the long sides closed by two patches. The pyramid shrunk by 3/2 and
//! grown by 3/2.
//!
//! Built as that composition, the case's own definition, by cad-kernel's offsets. Rounding the
//! edges one at a time with `fillet::roll` is refused by the kernel: the first base edge is rolled
//! and taken off, but the second, which meets it at a corner, is not — "the ball of `face 0` with
//! `face 2` is larger than `face 2` can hold": the first round's piece has left the end face an
//! edge inside the band the second ball would roll over. The base's corners would want
//! `fillet::corner_among` placed by the caller, and the split apex, where four faces meet and no
//! ball touches all four, has no operation.
use cad_kernel::build::hull;
use cad_kernel::offset::{grown, shrunk, Join};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;

const HALF: [f64; 2] = [10., 6.];
const APEX: V = [0., 0., 12.];
const ROUND: f64 = 1.5;

pub fn build() -> Result<Brep, String> {
    let [a, b] = HALF;
    // the pyramid: the convex hull of its base's corners and its apex
    let solid = hull(&[[-a, -b, 0.], [a, -b, 0.], [a, b, 0.], [-a, b, 0.], APEX])?;
    // the union of every ball of radius ROUND inside it: shrunk by ROUND, then grown by it
    grown(&shrunk(&solid, ROUND, Join::Round)?, ROUND, Join::Round)
}
