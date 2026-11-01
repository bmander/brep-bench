//! The sheet of five faces of the cube from the origin to (10, 10, 10), its floor and four walls
//! with no top, thickened by 1 out of the cube, the gaps the moved faces leave closed by the
//! quarter cylinders of radius 1 about the tray's eight edges and the eighth balls of radius 1
//! about the floor's four corners.
//!
//! next.BREP thickens one face at a time, so the tray is thickened as the faces of the cube by
//! the kernel's shell grown outward, open at the top, which extends the moved faces until they
//! meet. Its eight outer edges, along the floor and up the corners, are then rounded to radius 1
//! together: each round lies on the cylinder of radius 1 about the tray's edge, 1 from both moved
//! faces, and where three meet at a floor corner, on the ball of radius 1 about the corner.
use crate::solids::offset::{shell, tray};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (cube, opening) = tray()?;
    let wall = shell(&cube, &opening, -1.0)?;
    let edges = [
        p(5.0, -1.0, -1.0), p(11.0, 5.0, -1.0), p(5.0, 11.0, -1.0), p(-1.0, 5.0, -1.0),
        p(-1.0, -1.0, 4.5), p(11.0, -1.0, 4.5), p(11.0, 11.0, 4.5), p(-1.0, 11.0, 4.5),
    ];
    Ok(fillet_edges(&wall, &edges, None, 1.0, false, None)?)
}
