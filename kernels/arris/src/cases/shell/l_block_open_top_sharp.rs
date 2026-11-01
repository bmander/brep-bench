//! The L-shaped plate through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), extruded 8
//! along +z, shelled inward 1 thick with its top open, every edge kept sharp: arris's shell joins
//! the moved faces sharp, the concave edge's two walls extended until they meet.
use crate::solids::face_through;
use crate::Refusal;
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Control, Frame, Point2, Point3, Vec3};
use arris::ops::{extrude, shell, ShellSide};
use arris::topo::{Body, Model};

const OUTLINE: [(f64, f64); 6] =
    [(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)];
const HEIGHT: f64 = 8.0;
const THICKNESS: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let l = plate(m, &OUTLINE, HEIGHT)?;
    let top = face_through(m, l, Point3::new(2.0, 2.0, HEIGHT))?;
    let (shelled, _) = shell(m, l, &[top], THICKNESS, ShellSide::Inward, &Control::NONE)?;
    Ok(shelled)
}

fn plate(m: &mut Model, outline: &[(f64, f64)], height: f64) -> Result<Body, Refusal> {
    let corner = |(x, y): (f64, f64)| Point2::new(x, y);
    // the outline closed back to its first corner
    let segments = outline[1..]
        .iter()
        .chain(&outline[..1])
        .map(|&p| ProfileSegment::LineTo(corner(p)))
        .collect();
    let sketch = Profile {
        plane: Frame::world(),
        outer: ProfileLoop::Path { start: corner(outline[0]), segments },
        holes: vec![],
    };
    let (plate, _) = extrude(m, &sketch, Vec3::z(), height, &Control::NONE)?;
    Ok(plate)
}
