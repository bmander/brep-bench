
//!
//! cadrs's Loft with guides: the square sketched at z = 0 and the octagon at z = 30, along straight
//! guides pairing them, two from each corner of the square to the ends of the octagon's cut side
//! facing it (so the wall between those two is the triangle, and between neighbouring corners'
//! guides the trapezoid).
use crate::bench::p3;
use crate::shared::loft::guided_loft;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, LoftSection, Loop, PathCurve, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    // the square's corners counter-clockwise from (20, −20), and the octagon's from (6, −14):
    // corner k faces the octagon's cut side from corner 2k to 2k + 1
    let square = [(20.0, -20.0), (20.0, 20.0), (-20.0, 20.0), (-20.0, -20.0)];
    let octagon = [
        (6.0, -14.0),
        (14.0, -6.0),
        (14.0, 6.0),
        (6.0, 14.0),
        (-6.0, 14.0),
        (-14.0, 6.0),
        (-14.0, -6.0),
        (-6.0, -14.0),
    ];
    let foot = Profile::of(Plane::xy(0.0), vec![Loop::rectangle(-20.0, -20.0, 20.0, 20.0)]);
    let top = Profile::of(Plane::xy(30.0), vec![Loop::polygon(&octagon.map(|(x, y)| [x, y]))]);
    let mut guides = Vec::new();
    for (i, &(x, y)) in square.iter().enumerate() {
        for &(u, v) in &octagon[2 * i..2 * i + 2] {
            guides.push(vec![PathCurve::bezier(&[p3(x, y, 0.0), p3(u, v, 30.0)])]);
        }
    }
    guided_loft(k, vec![LoftSection::Profile(foot), LoftSection::Profile(top)], guides)
}
