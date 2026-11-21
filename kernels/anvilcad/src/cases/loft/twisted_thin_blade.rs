//! A thin blade turned a whole turn, its corners on four helices.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's loft is ruled between two sections; it has no loft along guide curves, and a ruled loft between its end sections joins their corners straight, not along the helices".into())
}
