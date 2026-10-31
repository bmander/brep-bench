use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = zcyl(0.0, 0.0, 32.0, 0.0, 6.0)?;
    let hub = zcyl(0.0, 0.0, 15.0, 6.0, 16.0)?;
    let mut flange = cut(&union(&plate, &hub)?, &zcyl(0.0, 0.0, 8.0, -1.0, 17.0)?)?;
    for i in 0..6 {
        let a = (60.0 * i as f64).to_radians();
        flange = cut(&flange, &zcyl(24.0 * a.cos(), 24.0 * a.sin(), 3.0, -1.0, 7.0)?)?;
    }
    Ok(flange)
}
