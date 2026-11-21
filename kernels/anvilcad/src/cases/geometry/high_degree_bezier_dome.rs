//! A block whose top is a Bézier patch of degree 15 each way.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no way to build a solid bounded by a given B-spline surface: its B-spline faces come only from its own lofts and blends".into())
}
