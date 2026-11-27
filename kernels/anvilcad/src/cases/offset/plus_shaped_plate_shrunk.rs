//! A plus-shaped plate shrunk by 1.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's offset moves each face along its normal and extends the moved faces until they meet, keeping the solid's edges; it adds no round at a concave edge and drops no feature the offset consumes, so it cannot shrink this solid to every point at least that far inside".into())
}
