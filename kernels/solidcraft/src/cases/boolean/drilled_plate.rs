use crate::solids::*;

pub fn build() -> R<Body> {
    let mut plate = block((0.0, 0.0, 0.0), (100.0, 20.0, 10.0))?;
    for i in 1..=9 {
        plate = cut(&plate, &zcyl(10.0 * i as f64, 10.0, 2.0, -1.0, 11.0)?)?;
    }
    Ok(plate)
}
