//! A square bar 10 × 10 swept along two straight runs, (0, 0, 0) to (40, 0, 0) and on, turned
//! 120° to the left, to (20, 20√3, 0), mitered at the corner: the square drawn on the Right
//! plane (x = 0) centered on the origin, swept along the two lines drawn on the Top plane
//! (cadrs's sweep mitres a corner of its path).
use crate::bench::p2;
use crate::shared::sweep::sweep;
use crate::Refusal;
use cadrs_kernel::{BodyId, Curve2, DefaultKernel, Loop, PathCurve, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let section = Profile::of(Plane::yz(0.0), vec![Loop::rectangle(-5.0, -5.0, 5.0, 5.0)]);
    let corner = p2(40.0, 0.0);
    let path = vec![
        PathCurve::sketch(Plane::xy(0.0), Curve2::line(p2(0.0, 0.0), corner)),
        PathCurve::sketch(Plane::xy(0.0), Curve2::line(corner, p2(20.0, 20.0 * 3f64.sqrt()))),
    ];
    sweep(k, &section, path)
}
