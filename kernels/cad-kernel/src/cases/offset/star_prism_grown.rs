//! A five-pointed star prism, its points 10 from the z axis (the first on +y) and its inside
//! corners 5, 2 high, grown by 1/2: its five concave edges, at an oblique angle, stay sharp.
use crate::sketch::polygon;
use cad_kernel::build::prism;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // ten corners anticlockwise from (0, 10), alternately 10 and 5 out, 36° apart
    let corners: Vec<_> = (0..10).map(|k| {
        let r = if k % 2 == 0 { 10. } else { 5. };
        let (s, c) = (90. + 36. * k as f64).to_radians().sin_cos();
        [r * c, r * s, 0.]
    }).collect();
    grown(&prism(&polygon(&corners), 0., 2.)?, 0.5, Join::Round)
}
