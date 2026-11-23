//! A half cylinder — the half of the cylinder of radius 5 on the z axis from z = 0 to z = 6 where
//! x ≥ 0 — its curved rim, the half circle where its side meets its top, rounded with a radius
//! that grows linearly with the angle φ round the z axis, r = 3/2 + φ/π: 1 at (0, −5, 6), 1.5 at
//! (5, 0, 6) and 2 at (0, 5, 6).
//!
//! Refused. cad-kernel has no variable-radius round: `fillet::roll` rolls a ball of one radius
//! along the whole edge, and nothing in `fillet`, `rounds`, `spill` or `corners` takes a radius
//! that changes along it. Nor is this round a surface the kernel can make otherwise: where the
//! radius grows along a straight edge the round is an oblique circular cone, a right cone sheared
//! (`affine::Affine`, as `fillet/variable-radius-concave-edge` builds it), but here its section,
//! the quarter circle of radius r(φ) in the half plane at φ, is one shape scaled about the edge as
//! it turns about the axis — no affine image of a cone, cylinder or torus, and no sweep the kernel
//! carries a section along (`geom::Carry` moves it rigidly, by a line or a screw). Its wall would
//! have to be fitted outside the kernel.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no variable-radius round: fillet::roll rolls a ball of one radius along the whole edge, \
         no operation of cad-kernel takes a radius that changes along it (r = 3/2 + φ/π round the \
         half circle), and the round's surface — a quarter circle scaled about the edge as it \
         turns about the axis — is no affine image of a kernel surface nor a rigid sweep of a \
         section"
        .into())
}
