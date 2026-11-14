//! A face gear, the ring between radii 40 and 52 about the z axis whose face is a cone falling
//! 1.4 across it, hobbed whole by a tapered single-start worm on the line through (0, 30, 6)
//! along x, from x = 0 to 52: the worm turns by t about its axis while the gear turns by t/30,
//! t from 0 to 60π.
//!
//! The ring is its meridian quadrilateral revolved. The thread is its section in the half plane
//! through the worm's axis (between the cones 2.5 inside and outside the pitch cone, its sides
//! 20° from square to the axis) carried along the spiral on the pitch cone by the spiral's own
//! screw (cadrs's sweep with `SweepControl::Helix`: turned about the axis as the path turns,
//! moved along it and out from it as the cone widens, so the section stays in a plane through
//! the axis), a turn past each end, cut to its ends and placed. Then cadrs's cut by a moving
//! tool under the two turns (`Kernel::cut_swept`).
//!
//! Across this ring a patch of the thread's second flank's envelope comes out from under the
//! path of its crest's edge partway along the tooth: toward the ring's outside a tooth space
//! has that fifth face, and cadrs makes the space in two stretches (nine faces a space).
use crate::bench::{moved, p3, revolve, turned, v3, P3};
use crate::shared::swept::{cut_swept_exactly, z_axis};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, BoolOp, DefaultKernel, Helix, Kernel, Loop, Plane, Profile, RigidStep, SolidMotion, SweepControl, SweepSpec};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// The tapered worm's thread on the line through `at` along x, from x = `from` to `to`: built
/// about the z axis (the pitch radius 6 + z·tan 5° at height z), then turned by 90° about the
/// y axis and moved to `at`, as the helicon's cylindrical thread is (`shared::swept::worm_thread`).
fn tapered_thread(k: &mut DefaultKernel, at: P3, from: f64, to: f64) -> Result<BodyId, Refusal> {
    let (tan5, tan20) = (5f64.to_radians().tan(), 20f64.to_radians().tan());
    let pitch = |z: f64| 6.0 + z * tan5;
    let w = |u: f64| FRAC_PI_2 - u * tan20;
    let start = -TAU * ((-from / TAU).ceil() + 1.0);
    let mid = start - FRAC_PI_2;
    // the section's corners: u from the pitch cone, at the heights mid ∓ w(u)
    let corner = |u: f64, side: f64| {
        let z = mid + side * w(u);
        [pitch(z) + u, z]
    };
    let section = [corner(-2.5, -1.0), corner(2.5, -1.0), corner(2.5, 1.0), corner(-2.5, 1.0)];
    let profile = Profile::of(Plane::xz(0.0), vec![Loop::polygon(&section)]);
    let turns = ((to - start) / TAU).ceil() + 1.0;
    let spiral = Helix::conical(Axis::new(p3(0.0, 0.0, mid), v3(0.0, 0.0, 1.0)), pitch(mid), pitch(mid + TAU * turns), TAU * turns, 0.0, turns);
    let mut spec = SweepSpec::solid(spiral.path());
    spec.control = SweepControl::Helix(spiral);
    let thread = k.sweep_with(&profile, &spec)?.first()?;
    let ends = crate::bench::cylinder(k, p3(0.0, 0.0, from), v3(0.0, 0.0, 1.0), 20.0, to - from)?;
    let thread = k.boolean(BoolOp::Intersect, thread, &[ends])?.first()?;
    let thread = turned(k, thread, Axis::new(P3::origin(), v3(0.0, 1.0, 0.0)), 90.0)?;
    moved(k, thread, at.coords)
}

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let quad = [[40.0, -9.0], [52.0, -9.0], [52.0, -1.7], [40.0, -0.3]];
    let ring = revolve(k, &Profile::of(Plane::xz(0.0), vec![Loop::polygon(&quad)]), z_axis(), None)?;
    let at = p3(0.0, 30.0, 6.0);
    let thread = tapered_thread(k, at, 0.0, 52.0)?;
    let motion = SolidMotion::Rigid {
        steps: vec![
            RigidStep::Turn { axis: Axis::new(at, v3(1.0, 0.0, 0.0)), from: 0.0, to: 60.0 * PI },
            RigidStep::Turn { axis: z_axis(), from: 0.0, to: -TAU },
        ],
    };
    cut_swept_exactly(k, ring, thread, &motion)
}
