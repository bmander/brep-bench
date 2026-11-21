//! A block under a bicubic B-spline with a double knot.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no way to build a solid bounded by a given B-spline surface: its B-spline faces come only from its own lofts and blends".into())
}
