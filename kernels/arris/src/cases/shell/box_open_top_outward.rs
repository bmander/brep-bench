//! The box from the origin to (10, 8, 6) shelled outward 1 thick, open at its top, its outside
//! rounded: the box grown by 1 and cut off at z = 6, less the box.
//!
//! arris's shell joins the moved faces sharp at every edge. Grown by 1 with its edges sharp, each
//! of the box's convex edges below the rim leaves a sharp edge outside it, 1 from each of the two
//! moved faces; rounded to radius 1, that edge's round is the quarter cylinder of radius 1 about
//! the box's own edge, and where three such rounds meet at a bottom corner their corner is the
//! ball of radius 1 about the box's corner, so the rounded wall is every point within 1 of the box.
use crate::solids::{edge_through, face_through};
use crate::Refusal;
use arris::math::{Control, Point3};
use arris::ops::{fillet, primitive_box, shell, ShellSide};
use arris::topo::{Body, Model};

const SIZE: (f64, f64, f64) = (10.0, 8.0, 6.0);
const THICKNESS: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let (a, b, c) = SIZE;
    let (block, _) = primitive_box(m, Point3::origin(), Point3::new(a, b, c), &Control::NONE)?;
    let top = face_through(m, block, Point3::new(a / 2.0, b / 2.0, c))?;
    let (cup, _) = shell(m, block, &[top], THICKNESS, ShellSide::Outward, &Control::NONE)?;
    // the outside's edges below the rim: the four round its bottom, the four up its sides
    let t = THICKNESS;
    let points = [
        Point3::new(a / 2.0, -t, -t),
        Point3::new(a / 2.0, b + t, -t),
        Point3::new(-t, b / 2.0, -t),
        Point3::new(a + t, b / 2.0, -t),
        Point3::new(-t, -t, c / 2.0),
        Point3::new(a + t, -t, c / 2.0),
        Point3::new(a + t, b + t, c / 2.0),
        Point3::new(-t, b + t, c / 2.0),
    ];
    let edges = points
        .into_iter()
        .map(|p| edge_through(m, cup, p))
        .collect::<Result<Vec<_>, _>>()?;
    let (rounded, _) = fillet(m, cup, &edges, t, &Control::NONE)?;
    Ok(rounded)
}
