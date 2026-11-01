//! The solid between two discs of radius 5 at z = ±5, its side ruled between their rims a quarter
//! turn apart: a hyperboloid of one sheet built as a ruled surface.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel has no ruled surface between two curves, and its loft of B-spline nets \
         (`lofted`) carries no weights, so a net ruled between two rational circles has no exact \
         rims".into())
}
