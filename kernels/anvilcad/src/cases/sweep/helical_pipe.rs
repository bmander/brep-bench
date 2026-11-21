//! A pipe of radius 1 swept three turns along a helix of radius 10 rising 5 a turn, its ends flat.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no helical curve: its sweeps follow chains of lines and arcs only".into())
}
