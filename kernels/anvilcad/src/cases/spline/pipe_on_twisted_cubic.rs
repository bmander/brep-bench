//! A pipe of radius 8 swept along a cubic Bézier curve out of plane.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's profiles are lines and arcs only, and its sweeps follow lines and arcs only: it has no B-spline or Bézier curve to sweep along".into())
}
