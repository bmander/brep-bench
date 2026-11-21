//! A wheel carried along a spline, cutting with its rim.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no sweep of a solid, and no spline curve to carry anything along: its sweeps follow lines and arcs only".into())
}
