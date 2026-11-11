//! A cone frustum on the z axis, the disk of radius 6 at z = 0 up to the disk of radius 3 at z = 4, bored through by a hole of radius 1, shelled inward to a wall 1 thick, open at its top: the moved side and the moved bore meet at z = 11/3, below the top, so the cavity is a closed ring and the top stays whole.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::shell;
use crate::Refusal;
use brep_kernel::{make_cone_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let up = p(0.0, 0.0, 1.0);
    let frustum = make_cone_brep(p(0.0, 0.0, 0.0), up, 6.0, 3.0, 4.0)?;
    // The drill runs past both faces.
    let drill = make_cylinder_brep(p(0.0, 0.0, -1.0), up, 1.0, 6.0)?;
    let bored = cut(&frustum, &drill)?;
    shell(&bored, &[p(0.0, 2.0, 4.0)], 1.0)
}
