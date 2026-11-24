//! The outside of a pipe elbow whose bend is tighter than the pipe is thick: the disk of radius 20
//! about (0, 0, 18) in the plane x = 0 turned a quarter turn about the y axis, +z to +x, with a
//! straight leg of length 40 at each end, the cylinders of radius 20 about the line y = 0, z = 18
//! from x = −40 to 0 and about the line x = 18, y = 0 from z = −40 to 0, all joined.
use std::f64::consts::FRAC_PI_2;

use springrep::boolean::union;
use springrep::math::{Vec2, Vec3};
use springrep::primitive::cylinder;
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::{arc_to, e, frame};

pub fn build() -> Result<Brep, String> {
    // The disk reaches 2 past the bend axis, and springrep's revolve takes no profile that crosses
    // its axis, so the disk is cut at its chord on the axis and the part toward +z turned. The part
    // cut off, at most 2 past the axis, turns between −z and −x, and lies in both legs throughout:
    // a point d past the axis, turned by θ, stands nearer each leg's axis than d past it did. So the
    // union is the same.
    let h = (20f64 * 20. - 18. * 18.).sqrt();
    // the plane x = 0, its sketch x along +y and its sketch y along +z
    let plane = frame([0., 0., 0.], [1., 0., 0.], [0., 1., 0.])?;
    let p = |y: f64, z: f64| Vec2::new(y, z);
    let disk = Profile::outline(vec![
        arc_to(p(0., 18.), p(h, 0.), p(-h, 0.), true, "tube"),
        NamedSegment::line(p(-h, 0.), p(h, 0.), "axis"),
    ]);
    let turn = revolve(&disk, &plane, Vec3::ZERO, Vec3::Y, FRAC_PI_2, "turn").map_err(e)?;
    let first = cylinder(&frame([-40., 0., 18.], [1., 0., 0.], [0., 1., 0.])?, 20., 40., "first")
        .map_err(e)?;
    let second = cylinder(&frame([18., 0., -40.], [0., 0., 1.], [1., 0., 0.])?, 20., 40., "second")
        .map_err(e)?;
    let elbow = union(&turn, &first).map_err(e)?;
    union(&elbow, &second).map_err(e)
}
