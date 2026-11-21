//! The box from the origin to (40, 30, 10), the edge of its top and back rounded to radius 3, its
//! top moved up 6 carrying the round with it.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's direct edit moves faces along their normals, the faces beside them trimmed or extended to meet them; it has no move that carries a round along with the face it borders".into())
}
