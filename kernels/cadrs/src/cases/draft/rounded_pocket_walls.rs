//! The block from the origin to (40, 30, 15), with a pocket cut into its top: the box from
//! (8, 8, 5) to (32, 22, 15), its four vertical edges rounded to radius 4, its floor at z = 5.
//! All eight walls (four planes, four concave quarter cylinders) drafted 5°, pull +z, about the
//! neutral plane z = 5, each leaning away from the pocket's middle as it rises; the quarter
//! cylinders become quarter cones about the same axes, tangent to the planes beside them.
//!
//! The rectangle extruded; the pocket a rectangle sketched on the top face and extruded 10 down,
//! its vertical edges filleted, cut from the block; then cadrs's Draft of the pocket's walls
//! (leaning into the material: a positive angle).
use crate::bench::{block, cut, extrude, p3, v3};
use crate::shared::draft::{drafted, is_side};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Kernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(0.0, 0.0, 0.0), p3(40.0, 30.0, 15.0))?;
    let tool = extrude(k, &Profile::of(Plane::xy(15.0), vec![Loop::rectangle(8.0, 8.0, 32.0, 22.0)]), -10.0)?;
    let verticals = k.edges_at(tool,
        &[p3(8.0, 8.0, 10.0), p3(32.0, 8.0, 10.0), p3(32.0, 22.0, 10.0), p3(8.0, 22.0, 10.0)],
    )?;
    let rounded = k.fillet(tool, &verticals, 4.0)?.bodies[0];
    let part = cut(k, b, rounded)?;
    // The pocket's walls: the sides whose middles lie over the pocket's floor.
    let walls = k.faces_where(part, &|f| {
        is_side(f) && (7.0..33.0).contains(&f.center.x) && (7.0..23.0).contains(&f.center.y)
    })?;
    drafted(k, part, walls, 5.0, v3(0.0, 0.0, 1.0), p3(0.0, 0.0, 5.0))
}
