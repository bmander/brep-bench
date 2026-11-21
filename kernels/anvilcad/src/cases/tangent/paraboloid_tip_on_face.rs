//! A block united with a solid paraboloid whose vertex touches its bottom face.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no paraboloid: its profiles are lines and arcs only, so it cannot revolve a parabola, and it has no paraboloid surface".into())
}
