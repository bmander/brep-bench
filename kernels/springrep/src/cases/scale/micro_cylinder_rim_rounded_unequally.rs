//! `fillet/elliptic-section-cylinder-rim` at a hundredth of its size: a cylinder of radius 0.05
//! from z = 0 to 0.1, the circle where its side meets its top rounded by a quarter ellipse reaching
//! 0.01 across the top and 0.03 down the side. Refused for the original's reason.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no round whose section is a quarter ellipse, set back by a different distance on each face: its fillets are circular, and its only other section blend, the conic of fillet_conic, takes one setback for both faces and blends only a straight edge between planes".into())
}
