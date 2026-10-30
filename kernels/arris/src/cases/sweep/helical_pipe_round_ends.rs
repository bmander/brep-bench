//! A pipe of radius 1 along three turns of a helix, each end closed by a half ball.
use arris::geom::{FrameLaw, Path, PathSegment, Profile, ProfileLoop, ProfileSegment};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{fuse, revolve, sweep, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::{FRAC_1_SQRT_2, TAU};

const HELIX_RADIUS: f64 = 10.0;
const PITCH: f64 = 5.0;
const TURNS: usize = 3;
const PIPE_RADIUS: f64 = 1.0;

/// A half ball of `radius` on the disc about `centre` square to `outward`, bulging along it: the
/// quarter disc in a plane through `outward`, turned about it.
fn half_ball(m: &mut Model, centre: Point3, outward: Vec3, radius: f64) -> Result<Body, OpError> {
    let across = Frame::from_z(centre, outward)?.x().into_inner();
    let (r, h) = (radius, radius * FRAC_1_SQRT_2);
    let quarter_disc = Profile {
        plane: Frame::new(centre, across.cross(&outward), across)?,
        outer: ProfileLoop::Path {
            start: Point2::origin(),
            segments: vec![
                ProfileSegment::LineTo(Point2::new(r, 0.0)),
                ProfileSegment::ArcTo { to: Point2::new(0.0, r), via: Point2::new(h, h) },
                ProfileSegment::LineTo(Point2::origin()),
            ],
        },
        holes: vec![],
    };
    Ok(revolve(m, &quarter_disc, Axis::new(centre, outward)?, TAU, &Control::NONE)?.0)
}

// workaround: no sphere primitive
pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let axis = Axis::z_at(Point3::origin());
    let start = Point3::new(HELIX_RADIUS, 0.0, 0.0);
    let end = Point3::new(HELIX_RADIUS, 0.0, PITCH * TURNS as f64);
    // The helix written turn by turn, a path of three one-turn helix arcs. As one arc of three
    // turns, the same pipe sweeps, but arris 0.9.1 then declines to fuse a ball onto it: "the
    // section of the NURBS surface and the sphere surface is degenerate: turning points of the
    // section that cannot be told apart". A wall of one or two turns it sections.
    let helix = Path {
        start,
        segments: (0..TURNS)
            .map(|_| PathSegment::Helix { axis, pitch: PITCH, angle: TAU })
            .collect(),
    };
    // after whole turns the helix leaves its end as it left its start: along (0, 2πr, pitch)
    let tangent = Vec3::new(0.0, TAU * HELIX_RADIUS, PITCH);
    let section = Profile {
        plane: Frame::new(start, tangent, Vec3::x())?,
        outer: ProfileLoop::Circle { center: Point2::origin(), radius: PIPE_RADIUS },
        holes: vec![],
    };
    // a spring's frame: the binormal held to the axis, so the section turns with the helix
    let law = FrameLaw::FixedBinormal(Vec3::z_axis());
    let (pipe, _) = sweep(m, &section, &helix, law, &Control::NONE)?;
    // each end closed by a half ball whose flat face is the end's cap, which the fuse removes
    let back = half_ball(m, start, -tangent, PIPE_RADIUS)?;
    let front = half_ball(m, end, tangent, PIPE_RADIUS)?;
    let (pipe, _) = fuse(m, pipe, back, &Control::NONE)?;
    let (pipe, _) = fuse(m, pipe, front, &Control::NONE)?;
    Ok(pipe)
}
