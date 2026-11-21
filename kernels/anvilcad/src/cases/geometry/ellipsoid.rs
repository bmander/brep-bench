//! A solid ellipsoid of semi-axes 6, 4 and 3.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no ellipsoid surface and no elliptic profile to revolve, and its scale refuses to stretch a sphere, which it would make elliptic".into())
}
