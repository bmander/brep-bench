//! A thick disc, counterbored, with a countersunk hole drilled from the counterbore's floor so
//! close to its wall that the hole and its countersink reach under the wall. The disc is the
//! cylinder of radius 100 about z from z = −20 to 20; the counterbore removes the cylinder of
//! radius 60 about z from z = 0 up through the top. The hole is the cylinder of radius 3.5 about
//! the vertical through (59.9, 0) from its flat bottom z = −15 up to z = 0, flush with the floor;
//! the countersink the cone of 90° about the same axis whose circle on z = 0 has radius 4, its
//! point 4 below. The solid is the disc less the counterbore, the hole and the countersink.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, disc, prism};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];
const DISC: f64 = 100.;
const HALF_THICK: f64 = 20.;
const BORE: f64 = 60.;
/// The hole's axis, its radius and its depth below the floor.
const HOLE_AT: f64 = 59.9;
const HOLE: f64 = 3.5;
const DEPTH: f64 = 15.;
/// The countersink's radius on the floor; at 90° included it reaches as deep as it is wide.
const SINK: f64 = 4.;

pub fn build() -> Result<Brep, String> {
    let blank = prism(&disc([0., 0., -HALF_THICK], UP, DISC), 0., 2. * HALF_THICK)?;
    // the counterbore from the floor up past the top
    let counterbore = prism(&disc([0., 0., 0.], UP, BORE), 0., HALF_THICK + 1.)?;
    let bored = combined(&blank, &counterbore, Op::Cut, 0.)?;
    // the hole from its flat bottom up to the floor's plane exactly, and the countersink from its
    // point up to its circle there: neither reaches above the floor, under the wall
    let hole = prism(&disc([HOLE_AT, 0., -DEPTH], UP, HOLE), 0., DEPTH)?;
    let sink = cone([HOLE_AT, 0., -SINK], UP, 0., SINK, SINK)?;
    let drilled = combined(&bored, &hole, Op::Cut, 0.)?;
    Ok(combined(&drilled, &sink, Op::Cut, 0.)?)
}
