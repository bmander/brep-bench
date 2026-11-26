//! A square-to-round transition (`loft/square-to-round` three times larger): the square
//! −60 ≤ x, y ≤ 60 at z = 0 lofted to the circle of radius 45 about the z axis at z = 90, each
//! side of the square joined by a flat triangle to the circle's point nearest it and each corner
//! by an oblique cone to the quarter circle between.
//!
//! cadrs's Loft with guides: the square sketched at z = 0 and the circle at z = 90, along straight
//! guides pairing them, two from each corner of the square to the points of the circle nearest
//! the sides beside it (so the wall between those two is the cone from the corner to the quarter
//! circle, and between neighbouring corners' guides the triangle to one point).
use crate::bench::p3;
use crate::shared::loft::guided_loft;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, LoftSection, Loop, PathCurve, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    // the square's corners counter-clockwise from (60, −60), and the circle's points nearest its
    // sides from (0, −45): corner k lies between points k and k + 1
    let square = [(60.0, -60.0), (60.0, 60.0), (-60.0, 60.0), (-60.0, -60.0)];
    let points = [(0.0, -45.0), (45.0, 0.0), (0.0, 45.0), (-45.0, 0.0)];
    let foot = Profile::of(Plane::xy(0.0), vec![Loop::rectangle(-60.0, -60.0, 60.0, 60.0)]);
    let top = Profile::of(Plane::xy(90.0), vec![Loop::circle([0.0, 0.0], 45.0)]);
    let mut guides = Vec::new();
    for (i, &(x, y)) in square.iter().enumerate() {
        for &(u, v) in [points[i], points[(i + 1) % 4]].iter() {
            guides.push(vec![PathCurve::bezier(&[p3(x, y, 0.0), p3(u, v, 90.0)])]);
        }
    }
    guided_loft(k, vec![LoftSection::Profile(foot), LoftSection::Profile(top)], guides)
}
