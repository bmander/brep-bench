//! The ellipsoid x²/36 + y²/16 + z²/9 ≤ 1 with everything below z = 0 cut away.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel has no ellipsoid, no non-uniform scale of a ball, and no way to close a \
         B-spline surface on its own seam and poles into a solid".into())
}
