//! The Utah teapot made a solid from its 32 bicubic Bézier patches.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no way to build a solid bounded by given Bézier surfaces: its B-spline faces come only from its own lofts and blends, and it sews no faces into a solid".into())
}
