//! A block with a rectangular pocket 10 deep (corners rounded to 4), the rim of the pocket's floor
//! rounded to radius 4, as tight as its corners.
//!
//! The block (from (−40, −30, 0) to (40, 30, 16)) less the pocket's rounded rectangle extruded from
//! the floor z = 6 up past the top; the floor's eight rim edges, by their middles at z = 6, rounded
//! by the kernel's fillet as one chain.
use crate::solids::boolean::cut;
use crate::solids::outline::{deg, Chain};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, make_box_brep, AffineTransform, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let mut c = Chain::new();
    c.line((-21.0, -15.0), (21.0, -15.0))?
        .arc((21.0, -11.0), 4.0, deg(270.0), deg(360.0), true)?
        .line((25.0, -11.0), (25.0, 11.0))?
        .arc((21.0, 11.0), 4.0, deg(0.0), deg(90.0), true)?
        .line((21.0, 15.0), (-21.0, 15.0))?
        .arc((-21.0, 11.0), 4.0, deg(90.0), deg(180.0), true)?
        .line((-25.0, 11.0), (-25.0, -11.0))?
        .arc((-21.0, -11.0), 4.0, deg(180.0), deg(270.0), true)?;
    let pocket = extrude_profile_brep(&c.curves, p(0.0, 0.0, 1.0), 11.0)?;
    let up = AffineTransform { elements: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 6.0, 0.0, 0.0, 0.0, 1.0] };
    let pocket = brep_kernel::transform_brep(&pocket, up, false)?;
    let block = make_box_brep(p(-40.0, -30.0, 0.0), 80.0, 60.0, 16.0)?;
    let pocketed = cut(&block, &pocket)?;
    Ok(fillet_edges(&pocketed, &c.middles_at(6.0), None, 4.0, false, None)?)
}
