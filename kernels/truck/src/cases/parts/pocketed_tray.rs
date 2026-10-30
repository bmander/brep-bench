//! "An open tray lying on its back, its opening facing −y. It is made in three steps. 1. A block
//! from (−36, 0, −24) to (36, 16, 24). 2. A pocket taken out of it: the box from (−33, 0, −21) to
//! (33, 13, 21). 3. Four standoffs joined to the tray, standing on the floor inside the pocket.
//! Each is a tube along y, outer radius 5 and inner radius 2, from y = 5 to y = 13. Their axes are
//! the lines parallel to y through (x, z) = (26, 14), (−26, 14), (−26, −14) and (26, −14). Each
//! tube's hole is blind: it runs from y = 5 down to the floor and stops there."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const BLOCK: ([f64; 3], [f64; 3]) = ([-36.0, 0.0, -24.0], [36.0, 16.0, 24.0]);
const POCKET: ([f64; 3], [f64; 3]) = ([-33.0, 0.0, -21.0], [33.0, 13.0, 21.0]);
const OUTER_RADIUS: f64 = 5.0; // of a standoff
const INNER_RADIUS: f64 = 2.0;
const STANDOFF_END: f64 = 5.0; // y; each stands on the floor's inside face, y = 13
const AXES: [(f64, f64); 4] = [(26.0, 14.0), (-26.0, 14.0), (-26.0, -14.0), (26.0, -14.0)];
/// How far a tool runs past the face it opens or joins, so that no face of the tool lies in a
/// face of the tray: truck-shapeops declines faces that lie in one plane.
const OVERRUN: f64 = 1.0;

fn block((lo, hi): ([f64; 3], [f64; 3])) -> Solid {
    primitive::cuboid(BoundingBox::from_iter([Point3::from(lo), Point3::from(hi)]))
}

/// `a` with `b` cut from it: truck-shapeops cuts by intersecting with the tool turned inside out.
fn cut(a: &Solid, mut b: Solid, what: &str) -> std::result::Result<Solid, Refused> {
    b.not();
    let tolerance = shapes::boolean_tolerance(a, &b);
    truck_shapeops::and(a, &b, tolerance)
        .ok_or_else(|| Refused::none_from(&format!("truck-shapeops `and` ({what})")))
}

pub fn build() -> Built {
    // the pocket, run out through the opening at y = 0
    let (mut lo, hi) = POCKET;
    lo[1] -= OVERRUN;
    let mut tray = cut(&block(BLOCK), block((lo, hi)), "the pocket")?;

    let floor = POCKET.1[1]; // the floor's inside face, y = 13
    for (x, z) in AXES {
        // the standoff's body, a cylinder from its end on into the floor (which is 3 thick), so
        // that it joins the tray through a volume and not along the floor's face
        let foot = Point3::new(x, STANDOFF_END, z);
        let length = floor + OVERRUN - STANDOFF_END;
        let body = shapes::cylinder(foot, Vector3::unit_y(), OUTER_RADIUS, length);
        let tolerance = shapes::boolean_tolerance(&tray, &body);
        tray = truck_shapeops::or(&tray, &body, tolerance).ok_or_else(|| {
            Refused::none_from(&format!("truck-shapeops `or` (the standoff at ({x}, {z}))"))
        })?;
        // then its blind hole, from past its open end to the floor's face: the body now covers
        // the floor there, so the hole's end lies in no face of the tray
        let foot = Point3::new(x, STANDOFF_END - OVERRUN, z);
        let length = floor - (STANDOFF_END - OVERRUN);
        let drill = shapes::cylinder(foot, Vector3::unit_y(), INNER_RADIUS, length);
        tray = cut(&tray, drill, &format!("the hole of the standoff at ({x}, {z})"))?;
    }
    Ok(tray.into())
}
