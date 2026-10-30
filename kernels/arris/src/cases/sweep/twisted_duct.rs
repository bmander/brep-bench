//! A duct along y from 0 to 40, a square at its start and a diamond at its end, each corner
//! running straight to its partner, so each long face is a twisted ruled surface.
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Control, Frame, Point2, Point3, Vec3};
use arris::ops::{loft, LoftKind, OpError};
use arris::topo::{Body, Model};

const LENGTH: f64 = 40.0;
// corners A, B, C, D of each end in its plane's coordinates: x along +x and the plane's own y
// along −z (a frame whose normal is +y and whose x is +x)
const SQUARE: [(f64, f64); 4] = [(10.0, 10.0), (-10.0, 10.0), (-10.0, -10.0), (10.0, -10.0)];
const DIAMOND: [(f64, f64); 4] = [(0.0, 12.0), (-12.0, 0.0), (0.0, -12.0), (12.0, 0.0)];

fn section(y: f64, corners: &[(f64, f64); 4]) -> Result<Profile, OpError> {
    let at = |(u, v): (f64, f64)| Point2::new(u, v);
    let segments = corners[1..].iter().chain(&corners[..1])
        .map(|&p| ProfileSegment::LineTo(at(p))).collect();
    Ok(Profile {
        plane: Frame::new(Point3::new(0.0, y, 0.0), Vec3::y(), Vec3::x())?,
        outer: ProfileLoop::Path { start: at(corners[0]), segments },
        holes: vec![],
    })
}

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    // a ruled loft joins each corner of the square to the diamond's corner of the same place in
    // its list, A to A′ and so on round: every span between them a straight line
    let sections = [section(0.0, &SQUARE)?, section(LENGTH, &DIAMOND)?];
    let (duct, _) = loft(m, &sections, LoftKind::Ruled, false, &Control::NONE)?;
    Ok(duct)
}
