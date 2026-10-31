use crate::solids::*;

pub fn build() -> R<Body> {
    revolve_z(rect(6.0, 0.0, 10.0, 4.0), 360.0)
}
