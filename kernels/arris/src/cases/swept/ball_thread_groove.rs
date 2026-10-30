//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical groove
//! cut round it by a ball of radius 1 whose centre runs along the helix of radius 5 about the
//! z axis from (5, 0, −2), turning right-handed and rising 4 a turn, for six turns, to (5, 0, 22).
//!
//! arris has no sweep of a solid, so the ball's sweep is built from what it is by elementary
//! geometry: every point within 1 of the helix. That is the tube the ball's great disc square to
//! the helix fills as it runs, arris's sweep of the disc along the helix (its turns 4 apart, more
//! than the ball's width 2, so it never meets itself), with half a ball on each end. The ball where
//! it starts lies wholly below z = −1 and where it ends wholly above z = 21, and so do the tube's
//! end discs (their centres 2 beyond the rod's ends, their radius 1), so the half balls are clear
//! of the rod and what the sweep takes from it is the tube's alone: the tube is the tool cut here.
use arris::geom::{FrameLaw, Path, PathSegment, Profile, ProfileLoop};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{cut, primitive_cylinder, sweep, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::TAU;

const ROD_RADIUS: f64 = 5.0;
const ROD_LENGTH: f64 = 20.0;
const BALL_RADIUS: f64 = 1.0;
const HELIX_RADIUS: f64 = 5.0;
const PITCH: f64 = 4.0;
const TURNS: f64 = 6.0;
/// The height the ball's centre starts at.
const START_Z: f64 = -2.0;

// workaround: no sweep of a solid
pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let axis = Axis::z_at(Point3::origin());
    let (rod, _) = primitive_cylinder(m, axis, ROD_RADIUS, ROD_LENGTH, &Control::NONE)?;
    let start = Point3::new(HELIX_RADIUS, 0.0, START_Z);
    // the helix as one arc of six turns: written turn by turn, six one-turn arcs, the tube sweeps
    // as well, but arris 0.9.1 then declines the cut ("the face arrangement is not a subdivision:
    // a section edge of f5 ends at a node nothing else reaches")
    let helix = Path {
        start,
        segments: vec![PathSegment::Helix { axis, pitch: PITCH, angle: TURNS * TAU }],
    };
    // the ball's great disc square to the helix where it starts: its tangent there is
    // (0, 2πr, pitch)
    let tangent = Vec3::new(0.0, TAU * HELIX_RADIUS, PITCH);
    let disc = Profile {
        plane: Frame::new(start, tangent, Vec3::x())?,
        outer: ProfileLoop::Circle { center: Point2::origin(), radius: BALL_RADIUS },
        holes: vec![],
    };
    // a spring's frame: the binormal held to the axis, so the disc turns with the helix
    let law = FrameLaw::FixedBinormal(Vec3::z_axis());
    let (tube, _) = sweep(m, &disc, &helix, law, &Control::NONE)?;
    let (grooved, _) = cut(m, rod, tube, &Control::NONE)?;
    Ok(grooved)
}
