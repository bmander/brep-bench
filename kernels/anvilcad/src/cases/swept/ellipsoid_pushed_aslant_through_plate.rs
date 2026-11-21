//! An ellipsoid pushed aslant wholly through a plate.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no ellipsoid (no elliptic profile to revolve, and its scale refuses to stretch a sphere), and no sweep of a solid".into())
}
