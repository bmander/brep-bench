//! An L-shaped block shelled inward open at its step, the wall rising past the opening kept. The
//! outline through (0, 0), (20, 0), (20, 20), (15, 20), (15, 5) and (0, 5), extruded 20 along +z,
//! shelled inward to a wall 1 thick, open at the face y = 5, the corners sharp.
use crate::solids::face_through;
use crate::Refusal;
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Control, Frame, Point2, Point3, Vec3};
use arris::ops::{extrude, shell, ShellSide};
use arris::topo::{Body, Model};

const OUTLINE: [(f64, f64); 6] = [(0.0, 0.0), (20.0, 0.0), (20.0, 20.0), (15.0, 20.0), (15.0, 5.0), (0.0, 5.0)];

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let l = plate(m, &OUTLINE, 20.0)?;
    let step = face_through(m, l, Point3::new(7.5, 5.0, 10.0))?;
    let (shelled, _) = shell(m, l, &[step], 1.0, ShellSide::Inward, &Control::NONE)?;
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
