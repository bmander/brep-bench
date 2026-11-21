//! A hollow square duct narrowing as it bends a quarter turn.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's sweep carries its section unchanged; it has no sweep whose section scales along the path".into())
}
