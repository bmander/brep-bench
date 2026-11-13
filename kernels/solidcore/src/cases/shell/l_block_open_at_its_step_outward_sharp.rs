//! An L-shaped block shelled outward open at its step, the walls beyond the opening kept. The
//! outline through (0, 0), (20, 0), (20, 20), (15, 20), (15, 5) and (0, 5), extruded 20 along +z,
//! shelled outward to a wall 1 thick, open at the face y = 5, its edges sharp.
use crate::bench::{face_through, order, polygon};
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::{shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let outline = polygon(&[
        [0.0, 0.0, 0.0],
        [20.0, 0.0, 0.0],
        [20.0, 20.0, 0.0],
        [15.0, 20.0, 0.0],
        [15.0, 5.0, 0.0],
        [0.0, 5.0, 0.0],
    ]);
    let block = extrude(&outline, [0.0, 0.0, 1.0], 20.0, 0.0, DraftCorner::default(), true)?;
    let step = face_through(&block, [7.5, 5.0, 10.0])?;
    shell(&block, &order(&block)?, &[step], 1.0)?;
    Ok(block)
}
