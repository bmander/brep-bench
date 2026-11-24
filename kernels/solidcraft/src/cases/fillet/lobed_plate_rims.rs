use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::PI;

pub fn build() -> R<Body> {
    let turn = PI / 3.0;
    // each lobe's half sweep, and each hollow's centre's distance from the axis
    let alpha = ((3.0 * 87f64.sqrt() - 10.0) / 38.0).acos();
    let reach = 20.0 * 3f64.sqrt() + 6.0 * 29f64.sqrt();
    let mut segs = Vec::new();
    for k in 0..6 {
        let t = k as f64 * turn;
        // round the lobe's outer side, anticlockwise
        let lobe = v2(40.0 * t.cos(), 40.0 * t.sin());
        segs.push(Seg2::Arc { center: lobe, radius: 8.0, start: t - alpha, sweep: 2.0 * alpha });
        // then the hollow's inner side, clockwise about its centre, to the next lobe
        let h = t + turn / 2.0;
        let hollow = v2(reach * h.cos(), reach * h.sin());
        segs.push(Seg2::Arc { center: hollow, radius: 30.0, start: t + alpha + PI, sweep: turn - 2.0 * alpha });
    }
    let plate = extrude(&Plane::XY, Region2 { outer: Loop2 { segs }, holes: vec![] }, 0.0, 12.0)?;
    // both rims, each picked at the outermost point of the lobe on +x and followed all round
    let rims = solidcraft_kernel::tangent_chain(&plate, &[v3(48.0, 0.0, 12.0), v3(48.0, 0.0, 0.0)])?;
    solidcraft_kernel::fillet(&plate, &rims, 1.0)
}
