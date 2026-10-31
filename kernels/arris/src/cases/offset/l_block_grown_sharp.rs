//! The L-shaped plate through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), extruded 8
//! along +z, grown by 1 with its edges kept sharp: each face moved 1 outward, the faces extended to
//! meet.
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Control, Frame, Point2, Vec3};
use arris::ops::{extrude, offset_faces, OpError};
use arris::topo::{Body, Model};

const OUTLINE: [(f64, f64); 6] =
    [(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)];
const THICKNESS: f64 = 8.0;
const GROWTH: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let corner = |(x, y): (f64, f64)| Point2::new(x, y);
    // the outline closed back to its first corner
    let segments = OUTLINE[1..]
        .iter()
        .chain(&OUTLINE[..1])
        .map(|&p| ProfileSegment::LineTo(corner(p)))
        .collect();
    let sketch = Profile {
        plane: Frame::world(),
        outer: ProfileLoop::Path { start: corner(OUTLINE[0]), segments },
        holes: vec![],
    };
    let (plate, _) = extrude(m, &sketch, Vec3::z(), THICKNESS, &Control::NONE)?;
    // every face moved out along its normal, each edge where the moved faces meet
    let faces = m.faces(plate)?;
    let (grown, _) = offset_faces(m, plate, &faces, GROWTH, &Control::NONE)?;
    Ok(grown)
}
