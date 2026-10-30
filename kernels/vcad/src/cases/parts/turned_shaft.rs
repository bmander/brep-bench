//! A stepped shaft on the z axis: its section in the xz plane, radius 20 from z = 0 to z = 60 and
//! radius 12 on up to z = 160, the inside corner at the shoulder a quarter circle of radius 4
//! centred at (16, 64), turned a full turn about the z axis.
use vcad_kernel::vcad_kernel_math::{Point2, Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::{SketchProfile, SketchSegment};
use vcad_kernel::Solid;

pub fn build() -> Result<Solid, crate::Refusal> {
    let p = Point2::new;
    let line = |start, end| SketchSegment::Line { start, end };
    let section = vec![
        line(p(0.0, 0.0), p(20.0, 0.0)),
        line(p(20.0, 0.0), p(20.0, 60.0)),
        line(p(20.0, 60.0), p(16.0, 60.0)),
        // the round: from 270° to 180° about its centre, clockwise in the sketch's (x, z)
        SketchSegment::Arc {
            start: p(16.0, 60.0),
            end: p(12.0, 64.0),
            center: p(16.0, 64.0),
            ccw: false,
        },
        line(p(12.0, 64.0), p(12.0, 160.0)),
        line(p(12.0, 160.0), p(0.0, 160.0)),
        line(p(0.0, 160.0), p(0.0, 0.0)),
    ];
    // a sketch in the xz plane, its x along x and its y along z
    let section = SketchProfile::new(Point3::origin(), Vec3::x(), Vec3::z(), section)?;
    Ok(Solid::revolve(section, Point3::origin(), Vec3::z(), 360.0)?)
}
