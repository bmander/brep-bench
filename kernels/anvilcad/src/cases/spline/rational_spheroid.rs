//! A spheroid revolved from a rational quadratic B-spline, two quarter ellipses.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's profiles are lines, arcs and ellipses only, and its sweeps follow lines and arcs only: it has no B-spline or Bézier curve to revolve, and its revolve takes no ellipse".into())
}
