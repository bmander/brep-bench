//! A block's two rounded top edges deleted: the top and both sides meet again.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no face deletion: no direct edit takes faces out of a solid and closes the gap with the faces round them; its direct edit moves faces along their normals".into())
}
