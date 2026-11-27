//! A helical pipe of forty turns.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no helical curve: its sweeps follow chains of lines and arcs only".into())
}
