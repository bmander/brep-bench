//! A square plate from (−10, −10, 0) to (10, 10, 2) with a hole of radius 3 through it on the z
//! axis, both of the hole's rims rounded to radius 1, half the plate's thickness, in one fillet:
//! the two rounds take the hole's whole wall and meet, tangent, on its waist at z = 1.
use crate::solids::fillet::{on_circle, subtract};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(-10.0, -10.0, 0.0), 20.0, 20.0, 2.0)?;
    // The hole's cylinder runs past both faces.
    let hole = make_cylinder_brep(p(0.0, 0.0, -1.0), p(0.0, 0.0, 1.0), 3.0, 4.0)?;
    let holed = subtract(&plate, &hole)?;
    let rims = [on_circle(3.0, 2.0, 1.0), on_circle(3.0, 0.0, 1.0)];
    Ok(fillet_edges(&holed, &rims, None, 1.0, false, None)?)
}
