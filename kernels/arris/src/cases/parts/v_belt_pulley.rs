//! A pulley on the z axis: a hub of radius 12 from z = −14 to 14, a rim of radius 28 from −9 to
//! 9, a bore of radius 6, and a V groove round the rim, its root at radius 22.
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{cut, revolve, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::TAU;

const BORE_RADIUS: f64 = 6.0;
const HUB_RADIUS: f64 = 12.0;
const HUB_HALF_LENGTH: f64 = 14.0;
const RIM_RADIUS: f64 = 28.0;
const RIM_HALF_WIDTH: f64 = 9.0;
/// The groove's triangle in the xz plane: its root on the rim's middle, its mouth past the rim.
const GROOVE_ROOT: f64 = 22.0;
const GROOVE_MOUTH: f64 = 29.0;
const GROOVE_HALF_OPENING: f64 = 5.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    // the xz plane: u along x, v along z
    let xz = Frame::new(Point3::origin(), -Vec3::y(), Vec3::x())?;
    let axis = Axis::z_at(Point3::origin());
    let p = Point2::new;
    let outline = |corners: &[(f64, f64)]| Profile {
        plane: xz,
        outer: ProfileLoop::Path {
            start: p(corners[0].0, corners[0].1),
            segments: corners[1..]
                .iter()
                .chain(&corners[..1])
                .map(|&(u, v)| ProfileSegment::LineTo(p(u, v)))
                .collect(),
        },
        holes: vec![],
    };
    // the half section of hub, rim and bore, turned a full turn
    let section = outline(&[
        (BORE_RADIUS, -HUB_HALF_LENGTH),
        (HUB_RADIUS, -HUB_HALF_LENGTH),
        (HUB_RADIUS, -RIM_HALF_WIDTH),
        (RIM_RADIUS, -RIM_HALF_WIDTH),
        (RIM_RADIUS, RIM_HALF_WIDTH),
        (HUB_RADIUS, RIM_HALF_WIDTH),
        (HUB_RADIUS, HUB_HALF_LENGTH),
        (BORE_RADIUS, HUB_HALF_LENGTH),
    ]);
    let (pulley, _) = revolve(m, &section, axis, TAU, &Control::NONE)?;
    // the groove, its triangle turned a full turn and taken away
    let triangle = outline(&[
        (GROOVE_ROOT, 0.0),
        (GROOVE_MOUTH, -GROOVE_HALF_OPENING),
        (GROOVE_MOUTH, GROOVE_HALF_OPENING),
    ]);
    let (groove, _) = revolve(m, &triangle, axis, TAU, &Control::NONE)?;
    let (grooved, _) = cut(m, pulley, groove, &Control::NONE)?;
    Ok(grooved)
}
