//! A plate with an elliptic hole, the hole's top rim chamfered 1 on both faces.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's profiles are lines and arcs only, it has no elliptic cylinder, and its scale makes no round face elliptic, so it cannot make the elliptic hole".into())
}
