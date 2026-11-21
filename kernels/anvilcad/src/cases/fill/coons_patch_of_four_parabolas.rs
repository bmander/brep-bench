//! The region under the bilinear Coons patch of four parabolic arcs over the square from (0, 0) to (10, 10).
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no surface fill: it makes no Coons patch, and its ruled lofts join lines and arcs, not parabolas".into())
}
