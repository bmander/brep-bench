//! #3DBenchy, from its designer's CAD file.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep cannot build the reference's B-spline surfaces and surfaces of extrusion from their nets and curves (it makes no solid from a given B-spline surface, and its profiles are lines, arcs and ellipses only and its sweeps lines and arcs only), so it cannot rebuild the boat".into())
}
