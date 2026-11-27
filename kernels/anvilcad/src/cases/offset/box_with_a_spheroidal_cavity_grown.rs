//! A box with a spheroidal cavity grown by 1.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no ellipsoid surface, its revolve takes no ellipse, and its scale refuses to stretch a sphere, which it would make elliptic".into())
}
