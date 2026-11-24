//! The tip of a solid paraboloid, 10⁻⁴ high.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no paraboloid: its profiles are lines, arcs and ellipses only, so it cannot revolve a parabola, and it has no paraboloid surface".into())
}
