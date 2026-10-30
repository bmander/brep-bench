//! A tray: a block 72 × 16 × 48 pocketed from its face y = 0 to leave walls and a floor 3
//! thick, with four tubes of radii 5 and 2 standing 8 tall on the floor, their holes blind.
use arris::geom::{Profile, ProfileLoop};
use arris::math::{Control, Frame, Point2, Point3, Vec3};
use arris::ops::{cut, extrude, fuse_many, primitive_box, OpError};
use arris::topo::{Body, Model};

const BLOCK: ([f64; 3], [f64; 3]) = ([-36.0, 0.0, -24.0], [36.0, 16.0, 24.0]);
const POCKET: ([f64; 3], [f64; 3]) = ([-33.0, 0.0, -21.0], [33.0, 13.0, 21.0]);
const STANDOFF_OUTER: f64 = 5.0;
const STANDOFF_INNER: f64 = 2.0;
/// Each standoff's top, toward the opening, and how far it runs from there to the floor.
const STANDOFF_TOP: f64 = 5.0;
const STANDOFF_HEIGHT: f64 = 8.0;
/// The standoffs' axes, parallel to y, as (x, z).
const STANDOFFS: [(f64, f64); 4] = [(26.0, 14.0), (-26.0, 14.0), (-26.0, -14.0), (26.0, -14.0)];

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let corner = |[x, y, z]: [f64; 3]| Point3::new(x, y, z);
    let (block, _) = primitive_box(m, corner(BLOCK.0), corner(BLOCK.1), &Control::NONE)?;
    // the pocket's mouth lies in the block's face y = 0: arris drops the coincident faces
    let (pocket, _) = primitive_box(m, corner(POCKET.0), corner(POCKET.1), &Control::NONE)?;
    let (tray, _) = cut(m, block, pocket, &Control::NONE)?;
    // each standoff an annulus extruded from y = 5 down to the floor, its end flush with it, so
    // the fuse leaves the floor whole under the hole
    let mut bodies = vec![tray];
    for (x, z) in STANDOFFS {
        let annulus = Profile {
            plane: Frame::new(Point3::new(x, STANDOFF_TOP, z), Vec3::y(), Vec3::x())?,
            outer: ProfileLoop::Circle { center: Point2::origin(), radius: STANDOFF_OUTER },
            holes: vec![ProfileLoop::Circle { center: Point2::origin(), radius: STANDOFF_INNER }],
        };
        bodies.push(extrude(m, &annulus, Vec3::y(), STANDOFF_HEIGHT, &Control::NONE)?.0);
    }
    let (tray, _) = fuse_many(m, &bodies, &Control::NONE)?;
    Ok(tray)
}
