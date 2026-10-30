//! The solid a box fills as it slides without turning along a straight line: the box from the
//! origin to (10, 6, 4), slid along the vector (8, 5, 3).
//!
//! arris has no sweep of a solid, so the swept solid is built from the box and its faces. A point
//! the box passes over but that is not in the box where it starts is first reached by a face the
//! box leads with, one whose outward normal points along the slide: the faces x = 10, y = 6 and
//! z = 4 here. So the sweep is the box where it starts and those three faces each slid along the
//! line, an oblique prism apiece, which meet only on their sides (their volumes add up to the
//! box's and the three slanted bricks', and the box where it ends is the far end of all four).
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Control, Frame, Point2, Point3, Vec3};
use arris::ops::{extrude, fuse, primitive_box, OpError};
use arris::topo::{Body, Model};

const LENGTH: f64 = 10.0;
const WIDTH: f64 = 6.0;
const HEIGHT: f64 = 4.0;
const SLIDE: [f64; 3] = [8.0, 5.0, 3.0];

/// The rectangle from the origin to (`u`, `v`) in `plane`.
fn rectangle(plane: Frame, u: f64, v: f64) -> Profile {
    let p = Point2::new;
    Profile {
        plane,
        outer: ProfileLoop::Path {
            start: p(0.0, 0.0),
            segments: vec![
                ProfileSegment::LineTo(p(u, 0.0)),
                ProfileSegment::LineTo(p(u, v)),
                ProfileSegment::LineTo(p(0.0, v)),
                ProfileSegment::LineTo(p(0.0, 0.0)),
            ],
        },
        holes: vec![],
    }
}

// workaround: no sweep of a solid
pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (block, _) =
        primitive_box(m, Point3::origin(), Point3::new(LENGTH, WIDTH, HEIGHT), &Control::NONE)?;
    let slide = Vec3::new(SLIDE[0], SLIDE[1], SLIDE[2]);
    // the three leading faces, each in its own plane with its outward normal: u, v along the
    // face's edges from the corner nearest the origin
    let leading = [
        rectangle(Frame::new(Point3::new(LENGTH, 0.0, 0.0), Vec3::x(), Vec3::y())?, WIDTH, HEIGHT),
        rectangle(Frame::new(Point3::new(0.0, WIDTH, 0.0), Vec3::y(), Vec3::z())?, HEIGHT, LENGTH),
        rectangle(Frame::new(Point3::new(0.0, 0.0, HEIGHT), Vec3::z(), Vec3::x())?, LENGTH, WIDTH),
    ];
    // The prisms fused onto the box one at a time, each fuse merging the faces they share. All
    // four in one `fuse_many`, arris 0.9.1 declines: "kernel bug: an edge along a face's boundary
    // with no common block was not found".
    let mut swept = block;
    for face in &leading {
        // each face slid the whole length of the slide: a prism whose sides are parallelograms
        let (prism, _) = extrude(m, face, slide, slide.norm(), &Control::NONE)?;
        swept = fuse(m, swept, prism, &Control::NONE)?.0;
    }
    Ok(swept)
}
