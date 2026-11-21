//! Two cone frustums base to base, the upper made turned two radians about the axis, their rounded ridge deleted: the two cones meet again.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no face deletion: no direct edit takes faces out of a solid and closes the gap with the faces round them; its direct edit moves faces along their normals".into())
}
