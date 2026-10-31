use crate::solids::*;

pub fn build() -> R<Body> {
    let mut disc = zcyl(0.0, 0.0, 20.0, -5.0, 0.0)?;
    for i in 0..6 {
        let a = (60.0 * i as f64).to_radians();
        disc = cut(&disc, &zcyl(12.0 * a.cos(), 12.0 * a.sin(), 2.0, -6.0, 1.0)?)?;
    }
    Ok(disc)
}
