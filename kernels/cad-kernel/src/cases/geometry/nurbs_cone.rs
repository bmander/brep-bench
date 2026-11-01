//! A cone on the disc of radius 5 in the plane z = 0, its apex at (0, 0, 10), its side a 9 × 2
//! rational net whose second row collapses to the apex.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel builds a B-spline net only as a sheet (`sheet`, `grid_sheet`) or as a loft \
         capped by two planes (`lofted`, which carries no weights): it has no way to join a sheet \
         and a flat face into a solid".into())
}
