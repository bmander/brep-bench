//! `shell/bored-frustum-sealed-below-its-opening` with every length 3.1 times as long: a cone
//! frustum on the z axis, the disk of radius 18.6 at z = 0 up to the disk of radius 9.3 at
//! z = 12.4, bored through by a hole of radius 3.1, shelled inward to a wall 3.1 thick, open at its
//! top: the moved side and the moved bore meet at z = 34.1/3, below the top, so the cavity is a
//! closed ring and the top stays whole. Built as the original is, every length times 3.1.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::shell;
use crate::Refusal;
use brep_kernel::{make_cone_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let up = p(0.0, 0.0, 1.0);
    let frustum = make_cone_brep(p(0.0, 0.0, 0.0), up, 18.6, 9.3, 12.4)?;
    // The drill runs past both faces.
    let drill = make_cylinder_brep(p(0.0, 0.0, -3.1), up, 3.1, 18.6)?;
    let bored = cut(&frustum, &drill)?;
    shell(&bored, &[p(0.0, 6.2, 12.4)], 3.1)
}
