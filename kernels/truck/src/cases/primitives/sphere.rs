//! "A solid ball of radius 5 centred on the origin."
use std::f64::consts::PI;
use truck_modeling::*;

const RADIUS: f64 = 5.0;

pub fn build() -> crate::Built {
    // as truck's `sphere` example: the meridian from the north pole down through +x to the south
    // pole, a half turn about the y axis in three arcs, so that no edge runs round the equator
    let pole = builder::vertex(Point3::new(0.0, 0.0, RADIUS));
    let meridian: Wire =
        builder::rsweep(&pole, Point3::origin(), Vector3::unit_y(), Rad(PI), 3);
    // `builder::cone` turns it once about the z axis and closes the poles; the meridian runs
    // down, against the axis, or the ball comes out inside out
    let shell = builder::cone(&meridian, Vector3::unit_z(), Rad(2.0 * PI), 4);
    let ball = Solid::new(vec![shell]);
    Ok(ball.into())
}
