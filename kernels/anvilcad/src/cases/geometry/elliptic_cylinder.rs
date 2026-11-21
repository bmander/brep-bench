//! A solid elliptic cylinder of semi-axes 5 and 3, 10 high.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's profiles are lines and arcs only, it has no elliptic cylinder, and its scale makes no round face elliptic, so it cannot make the elliptic cylinder".into())
}
