//! A slab lofted through rectangles whose corners follow four cubic Bézier guides.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's loft is ruled between two sections; it has no loft along guide curves".into())
}
