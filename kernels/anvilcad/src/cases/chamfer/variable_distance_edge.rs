//! A cube, one edge chamfered with a distance that grows along it on one face.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no variable-distance chamfer: its chamfers keep their distances along the whole edge".into())
}
