//! A prism on a closed cubic B-spline, the rim of its top chamfered 1 on both faces.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's profiles are lines and arcs only, so it cannot extrude a spline outline, and it has no other way to make the prism's B-spline side".into())
}
