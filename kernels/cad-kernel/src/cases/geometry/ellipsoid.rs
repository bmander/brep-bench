//! A solid ellipsoid about the origin of semi-axes 6, 4 and 3 along x, y and z.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // three unequal semi-axes: no surface of revolution, and no tensor net of a closed rational
    // surface can be built into a solid here
    Err("cad-kernel has no ellipsoid, no non-uniform scale of a ball, and no way to close a \
         B-spline surface on its own seam and poles into a solid".into())
}
