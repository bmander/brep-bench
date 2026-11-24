//! The solid the torus about (3, 0, 2) (radii 1 and 0.16) fills as it tumbles about two skew
//! axes, for t from −90° to 90°.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep's sweep of a solid is exact only under a slide, a turn or a screw. Under two turns about skew axes no line of the torus stays in contact, so the swept solid's boundary is an envelope of the torus's poses of no simpler kind, which it cannot make exactly".into())
}
