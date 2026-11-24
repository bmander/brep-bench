//! A block with an L-shaped pocket 5 deep (convex corners rounded to 1, the re-entrant corner to 3),
//! the rim of the pocket's floor rounded to radius 1, as tight as its convex corners.
//!
//! The block (from (−20, −20, 0) to (20, 20, 8)) less the pocket's outline (twelve tangent pieces,
//! exactly as the case gives them) extruded from the floor z = 3 up past the top; the floor's
//! twelve rim edges, by their middles at z = 3, rounded by the kernel's fillet as one chain.
use crate::solids::boolean::cut;
use crate::solids::outline::{deg, Chain};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let mut c = Chain::new();
    c.line((-11.0, -12.0), (11.0, -12.0))?
        .arc((11.0, -11.0), 1.0, deg(270.0), deg(360.0), true)?
        .line((12.0, -11.0), (12.0, -1.0))?
        .arc((11.0, -1.0), 1.0, deg(0.0), deg(90.0), true)?
        .line((11.0, 0.0), (3.0, 0.0))?
        .arc((3.0, 3.0), 3.0, deg(270.0), deg(180.0), false)?
        .line((0.0, 3.0), (0.0, 11.0))?
        .arc((-1.0, 11.0), 1.0, deg(0.0), deg(90.0), true)?
        .line((-1.0, 12.0), (-11.0, 12.0))?
        .arc((-11.0, 11.0), 1.0, deg(90.0), deg(180.0), true)?
        .line((-12.0, 11.0), (-12.0, -11.0))?
        .arc((-11.0, -11.0), 1.0, deg(180.0), deg(270.0), true)?;
    let pocket = extrude_profile_brep(&c.curves, p(0.0, 0.0, 1.0), 6.0)?;
    let pocket = brep_kernel::transform_brep(&pocket, brep_kernel::AffineTransform { elements: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 3.0, 0.0, 0.0, 0.0, 1.0] }, false)?;
    let block = make_box_brep(p(-20.0, -20.0, 0.0), 40.0, 40.0, 8.0)?;
    let pocketed = cut(&block, &pocket)?;
    Ok(fillet_edges(&pocketed, &c.middles_at(3.0), None, 1.0, false, None)?)
}
