use crate::solids::*;

pub fn build() -> R<Body> {
    union(&zcyl(0.0, 0.0, 5.0, 0.0, 10.0)?, &zcyl(0.0, 0.0, 3.0, 10.0, 20.0)?)
}
