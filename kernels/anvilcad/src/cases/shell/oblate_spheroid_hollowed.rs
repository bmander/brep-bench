//! An oblate spheroid hollowed to a wall 1 thick.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no ellipsoid surface, its revolve takes no ellipse, and its scale refuses to stretch a sphere, which it would make elliptic".into())
}
