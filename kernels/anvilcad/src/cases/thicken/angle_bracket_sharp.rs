//! A creased sheet thickened outside its crease, the corner kept sharp.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no sheet bodies: every solid it holds is closed, checked so before it is handed on, so it has no open sheet to thicken, and no thicken operation".into())
}
