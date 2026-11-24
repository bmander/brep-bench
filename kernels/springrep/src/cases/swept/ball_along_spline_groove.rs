//! The box (−6, −6, −8)–(36, 26, 0) less everything a ball of radius 3 passes through as its
//! center runs along the cubic Bézier curve with control points (0, 0, 0), (10, 0, 0), (20, 20, 0)
//! and (30, 20, 0).
//!
//! springrep's sweep of a solid declines this one (it takes no motion whose velocity changes over a
//! sphere face). A ball's sweep is the tube about its center's path, the circle of its radius
//! carried square to the curve (springrep's path sweep), with the ball at each end; the curve turns
//! no tighter than the ball, so the tube does not fold.
use springrep::boolean::difference;
use springrep::geom::{BSpline, Curve};
use springrep::math::Vec2;
use springrep::primitive::sphere;
use springrep::profile::Profile;
use springrep::sweep::along_curve;
use springrep::topo::Brep;

use crate::shared::swept::{boxed, unite, v3};
use crate::shared::{at, e, frame};

pub fn build() -> Result<Brep, String> {
    let block = boxed([-6.0, -6.0, -8.0], [36.0, 26.0, 0.0], "block")?;
    let ctrl = vec![v3(0.0, 0.0, 0.0), v3(10.0, 0.0, 0.0), v3(20.0, 20.0, 0.0), v3(30.0, 20.0, 0.0)];
    let path = Curve::BSpline(
        BSpline::new(3, vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], ctrl).map_err(e)?,
    );
    // the ball's great circle square to the curve where it starts, along +x
    let start = frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])?;
    let tube = along_curve(&Profile::circle(Vec2::ZERO, 6.0), &start, &path, (0.0, 1.0), 0.0, "groove")
        .map_err(e)?;
    let first = sphere(&at([0.0, 0.0, 0.0]), 3.0, "ball").map_err(e)?;
    let last = sphere(&at([30.0, 20.0, 0.0]), 3.0, "ball").map_err(e)?;
    difference(&block, &unite(&[&first, &tube, &last])?).map_err(e)
}
