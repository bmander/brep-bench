use crate::solids::*;

pub fn build() -> R<Body> {
    let l = polygon(&[(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)]);
    extrude(&Plane::XY, l, 0.0, 8.0)
}
