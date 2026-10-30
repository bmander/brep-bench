//! The solid a box fills as it slides without turning along a straight line: the box from the
//! origin to (10, 6, 4), slid along the vector (8, 5, 3).
//!
//! vcad has no sweep of a solid, so the swept solid is built from the box and its faces. A point
//! the box passes over but that is not in the box where it starts is first reached by a face the
//! box leads with, one whose outward normal points along the slide: the faces x = 10, y = 6 and
//! z = 4 here. So the sweep is the box where it starts and those three faces each slid along the
//! line, an oblique prism apiece (vcad's extrusion of the face's rectangle along the slide); the
//! box where it ends is the far end of all four.
use vcad_kernel::vcad_kernel_math::{Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::SketchProfile;
use vcad_kernel::Solid;

const SIZE: [f64; 3] = [10.0, 6.0, 4.0];
const SLIDE: [f64; 3] = [8.0, 5.0, 3.0];

pub fn build() -> Result<Solid, crate::Refusal> {
    let [a, b, c] = SIZE;
    let slide = Vec3::new(SLIDE[0], SLIDE[1], SLIDE[2]);
    // vcad's box has its corner at the origin
    let start = Solid::cube(a, b, c);
    // each leading face as a rectangle sketched on it, its normal (x_dir × y_dir) outward
    let faces = [
        SketchProfile::rectangle(Point3::new(a, 0.0, 0.0), Vec3::y(), Vec3::z(), b, c),
        SketchProfile::rectangle(Point3::new(0.0, b, 0.0), Vec3::z(), Vec3::x(), c, a),
        SketchProfile::rectangle(Point3::new(0.0, 0.0, c), Vec3::x(), Vec3::y(), a, b),
    ];
    let mut swept = start;
    for face in faces {
        swept = swept.try_union(&Solid::extrude(face, slide)?)?;
    }
    Ok(swept)
}
