//! A plate from z = 0 to 12 whose outline, drawn with kentron's pen, is six lobes and six hollows
//! by turns, each running on tangent into the next: the lobes arcs of radius 8 about the points 40
//! from the z axis at 0°, 60°, … 300°, each through 2α, α = arccos((3√87 − 10)/38); the hollows
//! concave arcs of radius 30, each through 2α − 60°. The pen starts where the first lobe, about
//! (40, 0), meets the hollow before it, α round from the lobe's outermost point. Both rims, each
//! one tangent chain of twelve edges, rounded to radius 1 in one operation.
use crate::bench::{edge_through, p};
use crate::shared::fillet::plan_prism;
use crate::Refusal;
use kentron::blend;
use kentron::sketch::Step::{self, Arc, Heading, Start};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let alpha = ((3.0 * 87f64.sqrt() - 10.0) / 38.0).acos();
    let lobe = 2.0 * alpha.to_degrees();
    let mut outline: Vec<Step> = vec![Start(40.0 + 8.0 * alpha.cos(), -8.0 * alpha.sin()), Heading(90.0 - alpha.to_degrees())];
    for _ in 0..6 {
        outline.push(Arc { radius: 8.0, angle: lobe });
        outline.push(Arc { radius: -30.0, angle: lobe - 60.0 });
    }
    let plate = plan_prism(m, 0.0, &outline, 12.0)?;
    // each rim over the first lobe's outermost point
    let top = edge_through(m, plate, p(48.0, 0.0, 12.0))?;
    let bottom = edge_through(m, plate, p(48.0, 0.0, 0.0))?;
    Ok(blend::fillet(m, plate, &[top, bottom], 1.0)?)
}
