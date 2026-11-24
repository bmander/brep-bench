//! A prism on a closed periodic quadratic B-spline.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's profiles are lines, arcs and ellipses only, so it cannot extrude a B-spline outline".into())
}
