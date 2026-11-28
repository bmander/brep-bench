use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::PI;

/// The half disc of radius 5 on y ≥ 0 extruded from z = 0 to 10, and every one of its edges
/// rounded to 1 in one call, each picked at its middle.
pub fn build() -> R<Body> {
    let half_disc = Loop2 {
        segs: vec![
            Seg2::Line { a: v2(-5.0, 0.0), b: v2(5.0, 0.0) },
            Seg2::Arc { center: Vec2::ZERO, radius: 5.0, start: 0.0, sweep: PI },
        ],
    };
    let bar = extrude(&Plane::XY, Region2 { outer: half_disc, holes: vec![] }, 0.0, 10.0)?;
    let mut edges = Vec::new();
    for z in [0.0, 10.0] {
        // the end's half-circle rim, and its edge on the flat face
        edges.extend([v3(0.0, 5.0, z), v3(0.0, 0.0, z)]);
    }
    // the two straight edges where the round face meets the flat one
    edges.extend([v3(5.0, 0.0, 5.0), v3(-5.0, 0.0, 5.0)]);
    solidcraft_kernel::fillet(&bar, &edges, 1.0)
}
