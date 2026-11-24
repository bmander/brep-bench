//! A plate, the cylinder of radius 8 on the z axis from z = −4 to z = 0, with a boss standing on
//! it, the cylinder of radius 3 from z = 0 to z = 8; the boss's side and top then deleted, the
//! plate's top closing over the circle of radius 3 at the boss's foot.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{cylinder, deleting, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let plate = cylinder([0., 0., -4.], 8., 4.)?;
    let b = combined(&plate, &cylinder([0., 0., 0.], 3., 8.)?, Op::Union, 0.)?;
    let boss = faces_through(&b, &[[3., 0., 4.], [1., 0., 8.]])?;
    deleting(&b, &boss)
}
