//! The octahedron |x| + |y| + |z| ≤ 10 with all twelve of its edges chamfered, distance √6 on both
//! faces of each, four chamfers meeting at each corner and the corner cut flat by a square.
use vcad_kernel::vcad_kernel_math::{Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::SketchProfile;
use vcad_kernel::Solid;

const SIZE: f64 = 10.0;
const CHAMFER: f64 = 2.449489742783178; // √6

/// The octahedron |x| + |y| + |z| ≤ 10. vcad has no octahedron among its primitives, nor a solid
/// made of given planes or faces, so it is the common part of four slabs, `−10 ≤ s · p ≤ 10` for
/// `s` each of (1, 1, 1), (1, 1, −1), (1, −1, 1) and (−1, 1, 1): each a square sketched far wider
/// than the octahedron in the plane of one face, extruded across to the plane of the face opposite.
pub fn octahedron() -> Result<Solid, crate::Refusal> {
    let slab = |s: Vec3| -> Result<Solid, crate::Refusal> {
        // the square's sides, across s; it is 6·SIZE wide about the foot of the origin on s·p = −10
        let across = s.cross(Vec3::z());
        let up = s.cross(across);
        let (across, up) = (across / across.norm(), up / up.norm());
        let foot = Point3::origin() + s * (-SIZE / s.dot(s));
        let corner = foot - (across + up) * (3.0 * SIZE);
        let square = SketchProfile::rectangle(corner, across, up, 6.0 * SIZE, 6.0 * SIZE);
        Ok(Solid::extrude(square, s * (2.0 * SIZE / s.dot(s)))?)
    };
    let mut solid = slab(Vec3::new(1.0, 1.0, 1.0))?;
    for s in [Vec3::new(1.0, 1.0, -1.0), Vec3::new(1.0, -1.0, 1.0), Vec3::new(-1.0, 1.0, 1.0)] {
        solid = solid.try_intersection(&slab(s)?)?;
    }
    Ok(solid)
}

pub fn build() -> Result<Solid, crate::Refusal> {
    // every edge in one chamfer, each corner closed by the face through its faces' trimmed corners
    Ok(octahedron()?.chamfer(CHAMFER)?)
}
