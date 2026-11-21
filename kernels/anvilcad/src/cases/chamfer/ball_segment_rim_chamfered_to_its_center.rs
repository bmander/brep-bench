//! A ball cut flat above its middle, its rim chamfered 4 on the flat (to its center) and 1 on the ball.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's chamfers set back one distance on both faces, or one distance and an angle; it has no chamfer of two distances, and on a curved face no angle gives the second".into())
}
