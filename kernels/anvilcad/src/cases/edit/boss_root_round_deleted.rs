//! A boss's rounded root deleted: the boss stands on the plate with a sharp foot.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no face deletion: no direct edit takes faces out of a solid and closes the gap with the faces round them; its direct edit moves faces along their normals".into())
}
