//! A cube of side 10 between the origin and (10, 10, 10), the edge from (10, 10, 0) to
//! (10, 10, 10) rounded with a radius growing linearly along it, r = 1 + z/10: the kernel's
//! variable-radius fillet under a linear law over the edge's length, from 1 at its end on z = 0
//! to 2 at its end on z = 10.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges_variable_law, make_box_brep, BrepSolid, LawSegment, RadiusLaw};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cube = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 10.0, 10.0)?;
    let law = RadiusLaw::from_segments(&[LawSegment::Linear {
        length: 10.0,
        start_radius: 1.0,
        end_radius: 2.0,
    }])?;
    Ok(fillet_edges_variable_law(&cube, &[p(10.0, 10.0, 5.0)], None, &law, false, None)?)
}
