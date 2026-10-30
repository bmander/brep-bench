//! An L-shaped plate: its outline in the xy plane extruded 8 along +z.
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Control, Frame, Point2, Vec3};
use arris::ops::{extrude, OpError};
use arris::topo::{Body, Model};

const OUTLINE: [(f64, f64); 6] =
    [(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)];
const THICKNESS: f64 = 8.0;

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
    Ok(plate)
}
