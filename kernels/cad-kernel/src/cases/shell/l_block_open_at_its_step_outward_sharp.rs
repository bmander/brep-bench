//! An L-shaped block shelled outward open at its step, the walls beyond the opening kept. The
//! outline through (0, 0), (20, 0), (20, 20), (15, 20), (15, 5) and (0, 5), extruded 20 along +z,
//! shelled outward to a wall 1 thick, open at the face y = 5, its edges sharp.
use crate::sketch::{faces_through, polygon};
use cad_kernel::build::prism;
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let l = polygon(&[[0., 0., 0.], [20., 0., 0.], [20., 20., 0.], [15., 20., 0.], [15., 5., 0.], [0., 5., 0.]]);
    let body = prism(&l, 0., 20.)?;
    let open = faces_through(&body, &[[7.5, 5., 10.]])?;
    shelled(&body, &open, 1., Side::Out, Join::Sharp)
}
