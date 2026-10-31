//! A transition from the square −20 ≤ x, y ≤ 20 at z = 0 to the octagon at z = 30 that is the
//! square 28 × 28 with its corners cut off (corners (6, −14), (14, −6) and so on round): each
//! side of the square joined to the octagon's side parallel to it by a flat trapezoid, and each
//! corner of the square to the cut side facing it by a flat triangle. The convex hull of the two.
//!
//! arris's `loft` matches its sections' segments by arc length, so it cannot pair a corner of
//! the square with a whole side of the octagon. The hull is the common of two ruled lofts, each
//! of four flat sides: the trapezoids lie on the sides of the loft from the square to the square
//! 28 × 28 (the octagon's sides facing ±x and ±y are on that square's), and the triangles on the
//! sides of the loft from the square |x| + |y| ≤ 40 at z = 0 (on whose sides the first square's
//! corners lie) to |x| + |y| ≤ 20 at z = 30 (on whose sides the octagon's cut sides lie). The
//! second is lofted a little further each way, along the same planes, so that only the first
//! gives the floor and the top.
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Control, Frame, Point2, Point3, Vec3};
use arris::ops::{common, loft, LoftKind, OpError};
use arris::topo::{Body, Model};

const HEIGHT: f64 = 30.0;
/// The square's half-width at the foot, and the octagon's across its sides facing ±x and ±y.
const FOOT_HALF: f64 = 20.0;
const TOP_HALF: f64 = 14.0;
/// Half the diagonal-facing square's width along the axes at the foot, |x| + |y| ≤ 40 (through
/// the foot's corners), and at the top, |x| + |y| ≤ 20 (along the octagon's cut sides).
const FOOT_REACH: f64 = 40.0;
const TOP_REACH: f64 = 20.0;
/// How far past each end the diagonal-facing loft runs.
const MARGIN: f64 = 5.0;

/// The closed polygon through `corners` in the plane z = `z`, counter-clockwise about +z.
fn polygon(z: f64, corners: &[(f64, f64)]) -> Result<Profile, OpError> {
    let at = |(x, y): (f64, f64)| Point2::new(x, y);
    let segments = corners[1..].iter().chain(&corners[..1])
        .map(|&c| ProfileSegment::LineTo(at(c))).collect();
    Ok(Profile {
        plane: Frame::new(Point3::new(0.0, 0.0, z), Vec3::z(), Vec3::x())?,
        outer: ProfileLoop::Path { start: at(corners[0]), segments },
        holes: vec![],
    })
}

/// The square of half-width `h` at height `z`, its sides facing ±x and ±y.
fn square(z: f64, h: f64) -> Result<Profile, OpError> {
    polygon(z, &[(h, -h), (h, h), (-h, h), (-h, -h)])
}

/// The square |x| + |y| ≤ `r` at height `z`, its sides facing the diagonals.
fn diamond(z: f64, r: f64) -> Result<Profile, OpError> {
    polygon(z, &[(r, 0.0), (0.0, r), (-r, 0.0), (0.0, -r)])
}

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    // the frustum on the trapezoids' planes, its ends the floor and the top
    let straight = [square(0.0, FOOT_HALF)?, square(HEIGHT, TOP_HALF)?];
    let (frustum, _) = loft(m, &straight, LoftKind::Ruled, false, &Control::NONE)?;
    // the frustum on the triangles' planes, run MARGIN past both ends: its reach changes by
    // (TOP_REACH − FOOT_REACH)/HEIGHT for every unit of height
    let slope = (TOP_REACH - FOOT_REACH) / HEIGHT;
    let (below, above) = (-MARGIN, HEIGHT + MARGIN);
    let slanted = [
        diamond(below, FOOT_REACH + slope * below)?,
        diamond(above, FOOT_REACH + slope * above)?,
    ];
    let (chamfers, _) = loft(m, &slanted, LoftKind::Ruled, false, &Control::NONE)?;
    let (hull, _) = common(m, frustum, chamfers, &Control::NONE)?;
    Ok(hull)
}
