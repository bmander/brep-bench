//! A ring whose section is a closed periodic quadratic B-spline, revolved.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's profiles are lines and arcs only, so it cannot revolve a B-spline section".into())
}
