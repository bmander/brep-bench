//! An edge rounded with a radius that varies linearly along it.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no variable-radius fillet: its rounds have one radius along the whole edge".into())
}
