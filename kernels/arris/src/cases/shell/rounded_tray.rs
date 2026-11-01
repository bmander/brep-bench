//! The box from the origin to (20, 14, 10) with its vertical edges rounded to radius 2, shelled
//! inward 1 thick, thinner than its rounds, its top open.
use crate::solids::{edge_through, face_through};
use crate::Refusal;
use arris::math::{Control, Point3};
use arris::ops::{fillet, primitive_box, shell, ShellSide};
use arris::topo::{Body, Model};

const THICKNESS: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let p = plate(m)?;
    let top = face_through(m, p, Point3::new(SIZE.0 / 2.0, SIZE.1 / 2.0, SIZE.2))?;
    let (shelled, _) = shell(m, p, &[top], THICKNESS, ShellSide::Inward, &Control::NONE)?;
    Ok(shelled)
}

const SIZE: (f64, f64, f64) = (20.0, 14.0, 10.0);
const RADIUS: f64 = 2.0;

/// The box with its four vertical edges rounded.
fn plate(m: &mut Model) -> Result<Body, Refusal> {
    let (w, d, h) = SIZE;
    let (b, _) = primitive_box(m, Point3::origin(), Point3::new(w, d, h), &Control::NONE)?;
    let edges = [(0.0, 0.0), (w, 0.0), (w, d), (0.0, d)]
        .into_iter()
        .map(|(x, y)| edge_through(m, b, Point3::new(x, y, h / 2.0)))
        .collect::<Result<Vec<_>, _>>()?;
    let (rounded, _) = fillet(m, b, &edges, RADIUS, &Control::NONE)?;
    Ok(rounded)
}
