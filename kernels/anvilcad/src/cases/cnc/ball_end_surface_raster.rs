//! A plate finished by a ball-end mill in 944 aslant raster moves.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no sweep of a solid, and its sweeps follow only paths that run smoothly; the mill's 944 straight moves turn sharply at every point".into())
}
