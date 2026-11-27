//! A plate with a wide hole, grown by the hole's radius.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's offset moves each face along its normal and extends the moved faces until they meet, keeping the solid's edges; it adds no round about a convex edge or corner, so it cannot grow a solid to every point within a distance of it".into())
}
