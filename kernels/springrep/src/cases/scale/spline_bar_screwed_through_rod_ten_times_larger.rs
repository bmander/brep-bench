//! `swept/spline-bar-screwed-through-rod` ten times larger: the rod of radius 50 about the z axis
//! from z = 0 to 200 less everything a prism of closed B-spline section in the plane y = 0 passes
//! through as it is screwed about the z axis, rising 60 a turn. Built as the original is, every
//! length times 10 (the shared rod and reach cylinders written out here at the larger size).
use springrep::boolean::{difference, intersection};
use springrep::geom::BSpline;
use springrep::math::{Frame, Vec3};
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::primitive::cylinder;
use springrep::sweep::{extrude, solid};
use springrep::topo::Brep;

use crate::shared::{at, e};
use crate::shared::swept::{deg, screwing, v3};

pub fn build() -> Result<Brep, String> {
    let points = [
        (35.0, 0.0),
        (45.0, -15.0),
        (80.0, -15.0),
        (120.0, -10.0),
        (120.0, 10.0),
        (80.0, 15.0),
        (45.0, 15.0),
    ];
    // drawn in (x, z) on the plane y = −80, carried along +y
    let ctrl = (0..points.len() + 3)
        .map(|i| points[i % points.len()])
        .map(|(x, z)| v3(x, z, 0.0))
        .collect();
    let knots = (-3..=10).map(f64::from).collect();
    let curve = BSpline::periodic_cubic(knots, ctrl).map_err(e)?;
    let side = Segment::Spline {
        curve,
        reversed: false,
    };
    let section = Profile::outline(vec![NamedSegment::new(side, "side")]);
    let sketch = Frame::new(v3(0.0, -80.0, 0.0), -Vec3::Y, Vec3::X).map_err(e)?;
    let bar = extrude(&section, &sketch, -160.0, "bar").map_err(e)?;
    // No sweep of the whole tool: a screw keeps each point's distance from the z axis, so only the
    // tool within 55 of it reaches the rod, and only that part is swept.
    let reach = cylinder(&at([0.0, 0.0, -1000.0]), 55.0, 2000.0, "reach").map_err(e)?;
    let tip = intersection(&bar, &reach).map_err(e)?;
    let swept = solid(&tip, &screwing(60.0, deg(-90.0), deg(1350.0)), "screw").map_err(e)?;
    let rod = cylinder(&at([0.0, 0.0, 0.0]), 50.0, 200.0, "rod").map_err(e)?;
    difference(&rod, &swept).map_err(e)
}
