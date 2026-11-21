//! A square spire lofted to a point along four cubic Bézier ridges.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's loft is ruled between two sections; it has no loft along guide curves".into())
}
