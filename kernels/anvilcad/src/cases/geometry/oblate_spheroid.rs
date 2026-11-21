//! A solid oblate spheroid, radius 6 at its equator and 3 at its poles.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no ellipsoid surface and no elliptic profile to revolve, and its scale refuses to stretch a sphere, which it would make elliptic".into())
}
