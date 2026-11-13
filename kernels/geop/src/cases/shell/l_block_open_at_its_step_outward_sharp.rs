//! An L-shaped block shelled outward open at its step, the walls beyond the opening kept. The
//! outline through (0, 0), (20, 0), (20, 20), (15, 20), (15, 5) and (0, 5), extruded 20 along +z,
//! shelled outward to a wall 1 thick, open at the face y = 5, its edges sharp.
use crate::shared::hollow::{prism, shell};
use crate::{Bench, Refusal};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let outline = [[0.0, 0.0], [20.0, 0.0], [20.0, 20.0], [15.0, 20.0], [15.0, 5.0], [0.0, 5.0]];
    prism(b, "block", &outline, 20.0)?;
    shell(b, "walls", &[[7.5, 5.0, 10.0]], 1.0, true)
}
