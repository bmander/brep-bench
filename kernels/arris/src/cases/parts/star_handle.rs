//! A four-armed star handle: its outline of tip arcs, fillet arcs and the sides tangent to both
//! in the xz plane, extruded 12 along y with every side drafted 2° inward.
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Control, Frame, Point2, Point3, Vec3};
use arris::ops::{loft, LoftKind, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::FRAC_1_SQRT_2;

const TIP_DISTANCE: f64 = 33.1;
const TIP_RADIUS: f64 = 4.0;
const FILLET_DISTANCE: f64 = 15.9;
const FILLET_RADIUS: f64 = 3.0;
const HEIGHT: f64 = 12.0;
const DRAFT_DEGREES: f64 = 2.0;

/// The outline moved `inset` inward, on the plane y = `y` with u along x and v along z: each tip
/// circle's radius shrunk and each fillet circle's grown by `inset` about the same centres.
fn outline(y: f64, inset: f64) -> Result<Profile, OpError> {
    let (r_tip, r_fillet) = (TIP_RADIUS - inset, FILLET_RADIUS + inset);
    let d = FILLET_DISTANCE * FRAC_1_SQRT_2;
    let (tip, fillet) = (Point2::new(TIP_DISTANCE, 0.0), Point2::new(d, d));
    // The first arm's side toward the diagonal: the line n·x = p with the tip circle a radius on
    // its positive side and the fillet circle a radius on its negative side, so n·(tip − fillet)
    // is the radii's sum, the same at every inset: the sides move parallel to themselves. Of the
    // two such lines it is the one touching the tip circle on the diagonal's side of the axis.
    let between = tip - fillet;
    let turn = between.y.atan2(between.x) - ((r_tip + r_fillet) / between.norm()).acos();
    let n = Point2::new(turn.cos(), turn.sin()).coords;
    let on_tip = tip - n * r_tip;
    let on_fillet = fillet + n * r_fillet;
    // the arm's other side across its centre line, and each arm a quarter turn on from the last
    let mirror = |q: Point2| Point2::new(q.x, -q.y);
    let quarter = |q: Point2, k: usize| match k % 4 {
        0 => q,
        1 => Point2::new(-q.y, q.x),
        2 => Point2::new(-q.x, -q.y),
        _ => Point2::new(q.y, -q.x),
    };
    let tip_middle = Point2::new(TIP_DISTANCE + r_tip, 0.0);
    let fillet_middle = Point2::new(
        (FILLET_DISTANCE - r_fillet) * FRAC_1_SQRT_2,
        (FILLET_DISTANCE - r_fillet) * FRAC_1_SQRT_2,
    );
    // counter-clockwise from the fillet before the first arm: out along a side, round the tip's
    // outer arc through the axis, in along the other side, round the fillet's inner arc
    let mut segments = Vec::with_capacity(16);
    for k in 0..4 {
        segments.push(ProfileSegment::LineTo(quarter(mirror(on_tip), k)));
        segments.push(ProfileSegment::ArcTo {
            to: quarter(on_tip, k),
            via: quarter(tip_middle, k),
        });
        segments.push(ProfileSegment::LineTo(quarter(on_fillet, k)));
        segments.push(ProfileSegment::ArcTo {
            to: quarter(mirror(on_fillet), k + 1),
            via: quarter(fillet_middle, k),
        });
    }
    Ok(Profile {
        plane: Frame::new(Point3::new(0.0, y, 0.0), -Vec3::y(), Vec3::x())?,
        outer: ProfileLoop::Path { start: mirror(on_fillet), segments },
        holes: vec![],
    })
}

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    // workaround: no draft
    // arris has no draft: a ruled loft from the outline to its inset at the top joins each arc
    // to its coaxial partner, an exact cone, and each side to its parallel partner, a plane
    let inset = HEIGHT * DRAFT_DEGREES.to_radians().tan();
    let sections = [outline(0.0, 0.0)?, outline(HEIGHT, inset)?];
    let (handle, _) = loft(m, &sections, LoftKind::Ruled, false, &Control::NONE)?;
    Ok(handle)
}
