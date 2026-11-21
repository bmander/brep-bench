//! A post capped by a ball, the cap made turned 1.5 radians about the axis, its rounded rim deleted: the cylinder and the ball meet again.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no face deletion: no direct edit takes faces out of a solid and closes the gap with the faces round them; its direct edit moves faces along their normals".into())
}
