//! The box (−10, −10, −8)–(30, 25, 0) less everything a bull-nose mill passes through as its
//! upright axis runs along the cubic Bézier curve with control points (0, 0, 0), (30, 20, 0),
//! (−10, 20, 0) and (20, 0, 0), which crosses itself: a cylinder of radius 2 from z = −1.5 to 5,
//! its bottom corner rounded to 0.5.
//!
//! springrep's sweep of a solid is exact only under a slide, a turn or a screw. The mill is a solid
//! of revolution about an upright axis that slides along a level curve without turning, so at each
//! instant its distance from any point is least where the point lies in its section square to the
//! curve through its axis: the mill sweeps that section carried along the curve (springrep's path
//! sweep, which keeps a level curve's up direction), and itself where it starts and stops.
use std::f64::consts::{FRAC_PI_2, PI};

use springrep::boolean::difference;
use springrep::geom::{BSpline, Curve};
use springrep::sweep::along_curve;
use springrep::topo::Brep;

use crate::shared::swept::{arc, boxed, line, outline, shifted, unite, upright, v3};
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let block = boxed([-10.0, -10.0, -8.0], [30.0, 25.0, 0.0], "block")?;
    let meridian = outline(vec![
        line((0.0, -1.5), (1.5, -1.5), "bottom"),
        arc((1.5, -1.0), 0.5, -FRAC_PI_2, FRAC_PI_2, "round"),
        line((2.0, -1.0), (2.0, 5.0), "side"),
        line((2.0, 5.0), (0.0, 5.0), "top"),
        line((0.0, 5.0), (0.0, -1.5), "axis"),
    ]);
    let mill = upright(&meridian, 0.0, 0.0, "mill")?;
    let section = outline(vec![
        line((-1.5, -1.5), (1.5, -1.5), "floor"),
        arc((1.5, -1.0), 0.5, -FRAC_PI_2, FRAC_PI_2, "round"),
        line((2.0, -1.0), (2.0, 5.0), "wall"),
        line((2.0, 5.0), (-2.0, 5.0), "top"),
        line((-2.0, 5.0), (-2.0, -1.0), "wall"),
        arc((-1.5, -1.0), 0.5, PI, FRAC_PI_2, "round"),
    ]);
    let ctrl = vec![v3(0.0, 0.0, 0.0), v3(30.0, 20.0, 0.0), v3(-10.0, 20.0, 0.0), v3(20.0, 0.0, 0.0)];
    let path = Curve::BSpline(
        BSpline::new(3, vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], ctrl).map_err(e)?,
    );
    // the section square to the curve where it starts, heading along (3, 2, 0), sketch y up
    let heading = [3.0, 2.0, 0.0];
    let start = frame([0.0, 0.0, 0.0], heading, [-2.0, 3.0, 0.0])?;
    let channel = along_curve(&section, &start, &path, (0.0, 1.0), 0.0, "groove").map_err(|why| {
        format!("springrep's path sweep of the mill's section folds where the curve turns tighter than the mill: {why}")
    })?;
    let groove = unite(&[&mill, &channel, &shifted(&mill, v3(20.0, 0.0, 0.0))])?;
    difference(&block, &groove).map_err(e)
}
