use crate::solids::*;

pub fn build() -> R<Body> {
    let cube = block((0.0, 0.0, 0.0), (10.0, 10.0, 10.0))?;
    cut(&cube, &zcyl(5.0, 5.0, 3.0, -1.0, 11.0)?)
}
