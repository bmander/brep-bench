
//!
//! cadrs's Loft with guides: the rectangle sketched at z = 0 and again at z = 60, along the four
//! helices through its corners, each as quintic Bézier pieces matching the helix's points and
//! first and second derivatives at their ends (as cadrs's Helix feature draws one on a cone).
use crate::bench::p3;
use crate::shared::loft::guided_loft;
use crate::shared::sweep::conical_helix;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, LoftSection, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let (x, y) = (20.0f64, 0.2f64);
    let blade = |z: f64| Profile::of(Plane::xy(z), vec![Loop::rectangle(-x, -y, x, y)]);
    let guides = [(x, y), (-x, y), (-x, -y), (x, -y)]
        .iter()
        .map(|&(cx, cy)| conical_helix(p3(0.0, 0.0, 0.0), x.hypot(y), x.hypot(y), 60.0, cy.atan2(cx), 1.0))
        .collect();
    guided_loft(k, vec![LoftSection::Profile(blade(0.0)), LoftSection::Profile(blade(60.0))], guides)
}
