//! The box (−3, −3, −3)–(3, 3, 3) less everything the cube (−1, −1, −1)–(1, 1, 1) passes through
//! as it turns a whole turn about its long diagonal: a sealed cavity.
//!
//! springrep's sweep of a solid declines this one (faces round an edge do not pair off). A whole
//! turn sweeps everything whose circle about the diagonal meets the cube: its shadow on a
//! half-plane through the diagonal, turned. Measured along the diagonal by h from the center, every
//! section of the cube square to it holds the axis, so the shadow reaches from the axis out to the
//! section's farthest corner. Those corners lie on the three edges from each end of the diagonal,
//! which pass through the axis and sweep cones (ρ rising linearly to √(8/3) at |h| = 1/√3), and
//! between them on the six skew edges, each at distance √2 from the axis, which sweep the
//! hyperboloid ρ² = 2 + 2h²: a hyperbola's arc, exactly a rational quadratic Bézier. That meridian,
//! turned whole, is the cavity.
use std::f64::consts::TAU;

use springrep::boolean::difference;
use springrep::math::{Frame, Vec3};
use springrep::profile::{NamedSegment, Segment};
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, line, outline, v2, v3};

pub fn build() -> Result<Brep, String> {
    let block = boxed([-3.0, -3.0, -3.0], [3.0, 3.0, 3.0], "block")?;
    let (end, k) = (3f64.sqrt(), 1.0 / 3f64.sqrt());
    let r = (8.0f64 / 3.0).sqrt();
    // the hyperbola ρ² = 2 + 2h² from h = −k to k: its tangents there meet at (2/r, 0), and its
    // vertex (√2, 0) is the arc's middle, which sets the apex's weight
    let w = (r - 2f64.sqrt()) / (2f64.sqrt() - 2.0 / r);
    let skew = NamedSegment::new(
        Segment::Bezier {
            ctrl: vec![v2(r, -k), v2(2.0 / r, 0.0), v2(r, k)],
            weights: vec![1.0, w, 1.0],
        },
        "hyperboloid",
    );
    // sketch x out from the diagonal, y along it
    let meridian = outline(vec![
        line((0.0, -end), (r, -k), "cone"),
        skew,
        line((r, k), (0.0, end), "cone"),
        line((0.0, end), (0.0, -end), "axis"),
    ]);
    let along = v3(1.0, 1.0, 1.0) * (1.0 / end);
    let out = v3(1.0, -1.0, 0.0) * (1.0 / 2f64.sqrt());
    let sketch = Frame::new(Vec3::ZERO, out.cross(along), out).map_err(e)?;
    let cavity = revolve(&meridian, &sketch, Vec3::ZERO, along, TAU, "cavity").map_err(e)?;
    difference(&block, &cavity).map_err(e)
}
