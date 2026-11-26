//! The five-pointed star prism, the outline through ten points alternately 10 and 5 from the
//! origin, every 36° from (0, 10), 2 thick, grown by 1/2: its convex edges rounded, its five inside
//! corners' edges sharp.
use std::f64::consts::{FRAC_PI_2, PI};

use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::shell::prism;

pub fn build() -> Result<Brep, String> {
    let outline: Vec<[f64; 2]> = (0..10)
        .map(|i| {
            let angle = FRAC_PI_2 + i as f64 * PI / 5.;
            let rho = if i % 2 == 0 { 10. } else { 5. };
            [rho * angle.cos(), rho * angle.sin()]
        })
        .collect();
    let star = prism(&outline, &["wall"; 10], 2., "star")?;
    grown(&star, 0.5)
}
