//! An L-shaped bracket: its outline in the xz plane, a post 20 wide and 120 tall with an arm 20
//! deep along its top out to x = 100, the inside corner a quarter circle of radius 8 centred at
//! (28, 92), extruded from y = −5 to y = 5.
use vcad_kernel::vcad_kernel_math::{Point2, Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::{SketchProfile, SketchSegment};
use vcad_kernel::Solid;

const THICKNESS: f64 = 10.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let p = Point2::new;
    let line = |start, end| SketchSegment::Line { start, end };
    let outline = vec![
        line(p(0.0, 0.0), p(20.0, 0.0)),
        line(p(20.0, 0.0), p(20.0, 92.0)),
        // the round: from 180° to 90° about its centre, clockwise in the sketch's (x, z)
        SketchSegment::Arc {
            start: p(20.0, 92.0),
            end: p(28.0, 100.0),
            center: p(28.0, 92.0),
            ccw: false,
        },
        line(p(28.0, 100.0), p(100.0, 100.0)),
        line(p(100.0, 100.0), p(100.0, 120.0)),
        line(p(100.0, 120.0), p(0.0, 120.0)),
        line(p(0.0, 120.0), p(0.0, 0.0)),
    ];
    // a sketch in the plane y = −5, its x along x and its y along z
    let origin = Point3::new(0.0, -THICKNESS / 2.0, 0.0);
    let outline = SketchProfile::new(origin, Vec3::x(), Vec3::z(), outline)?;
    Ok(Solid::extrude(outline, Vec3::new(0.0, THICKNESS, 0.0))?)
}
