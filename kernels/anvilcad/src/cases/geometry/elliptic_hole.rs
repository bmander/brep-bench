//! A block with a hole of elliptic section bored through it.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's profiles are lines and arcs only, it has no elliptic cylinder, and its scale makes no round face elliptic, so it cannot make the elliptic hole".into())
}
