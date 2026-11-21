//! The solid inside one sheet of a hyperboloid of two sheets.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no hyperboloid surface, and its profiles are lines and arcs only, so it cannot revolve a hyperbola; a sheet of two is not ruled, so no loft makes it".into())
}
