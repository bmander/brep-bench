//! An elliptic cylinder, the rim of its top chamfered 2 on both faces.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's profiles are lines and arcs only, it has no elliptic cylinder, and its scale makes no round face elliptic, so it cannot make the elliptic cylinder".into())
}
