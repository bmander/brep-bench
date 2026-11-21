//! A half cylinder, its curved rim chamfered with a distance that grows round it.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no variable-distance chamfer: its chamfers keep their distances along the whole edge".into())
}
