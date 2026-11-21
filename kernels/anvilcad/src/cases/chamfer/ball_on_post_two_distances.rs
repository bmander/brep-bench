//! A post joined to a ball, the concave edge between them chamfered 2 down the post and 1 on the ball.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's chamfers set back one distance on both faces, or one distance and an angle; it has no chamfer of two distances, and on a curved face no angle gives the second".into())
}
