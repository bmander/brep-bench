//! A cube of side 10 from (−5, −5, 0) to (5, 5, 10), united with itself turned 0.001° about z.
//!
//! Refused at run time by cad-kernel 0.21.0's Boolean: each pair of sides 1.7·10⁻⁵ rad apart
//! crosses near the middle of its edge, and the complex cannot order the two planes round the
//! line they cross in ("edge 6 near [4.3e-5, −5, 0]: faces 1 and 22 (a plane and a plane)
//! coincide along it", `complex::Complex::new`). Tried as well, each refused the same way: the
//! operands the other way round, `boolean::combined_written` (whose flush step moves only parallel
//! planes), the turned cube made as a turned square's prism, and the union as the turned cube
//! joined to the cube less it.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::geom::Rigid;
use cad_kernel::topo::Brep;

const HALF: f64 = 5.;
const TURN_DEGREES: f64 = 0.001;

pub fn build() -> Result<Brep, String> {
    let cube = block([-HALF, -HALF, 0.], [HALF, HALF, 2. * HALF])?;
    let turned = cube.moved(&Rigid::turn([0., 0., 0.], [0., 0., 1.], TURN_DEGREES.to_radians()));
    Ok(combined(&cube, &turned, Op::Union, 0.)?)
}
