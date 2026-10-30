//! A solid arris has no primitive for, made the arris way and shaped like an arris operation.
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{revolve, OpError};
use arris::topo::provenance::Provenance;
use arris::topo::{Body, Model};
use std::f64::consts::TAU;

/// A ball of `radius` about `centre`: the half disc of that radius in the vertical plane through
/// `centre`, its diameter on the vertical axis, turned a full turn about that axis.
pub fn ball(
    m: &mut Model,
    centre: Point3,
    radius: f64,
    control: &Control<'_>,
) -> Result<(Body, Provenance), OpError> {
    // the plane through the centre whose u runs along x and v along z
    let meridian = Frame::new(centre, -Vec3::y(), Vec3::x())?;
    let half_disc = Profile {
        plane: meridian,
        outer: ProfileLoop::Path {
            start: Point2::new(0.0, -radius),
            segments: vec![
                ProfileSegment::LineTo(Point2::new(0.0, radius)),
                ProfileSegment::ArcTo {
                    to: Point2::new(0.0, -radius),
                    via: Point2::new(radius, 0.0),
                },
            ],
        },
        holes: Vec::new(),
    };
    revolve(m, &half_disc, Axis::z_at(centre), TAU, control)
}
