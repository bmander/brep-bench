//! A square column tapering as it turns half a turn, its corners on four conical spirals.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's loft is ruled between two sections; it has no loft along guide curves, and a ruled loft between its end squares joins their corners straight, not along the spirals".into())
}
