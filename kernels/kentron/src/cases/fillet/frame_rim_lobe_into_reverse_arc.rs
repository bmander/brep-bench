//! A block from z = 0 to 10 over a plan drawn with kentron's pen, thirteen pieces each running on
//! tangent into the next, anticlockwise from (5, 0): the south side to (30, 0), a concave quarter
//! circle of radius 5 dipping to (35, −5), the lobe (the lower half of the circle of radius 10
//! about (45, −5)), the straight rise to (55, 0), the concave reverse arc of radius 5 onto the
//! step at y = 5, and round the block's other sides with their corners rounded to 5. The rim of
//! its top, one tangent chain of thirteen edges, rounded to radius 2.
use crate::bench::{edge_through, p};
use crate::shared::fillet::plan_prism;
use crate::Refusal;
use kentron::blend;
use kentron::sketch::Step::{Arc, Forward, Start};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plan = [
        Start(5.0, 0.0),
        Forward(25.0), // south side
        Arc { radius: -5.0, angle: 90.0 }, // dip
        Arc { radius: 10.0, angle: 180.0 }, // lobe
        Forward(5.0), // rise
        Arc { radius: -5.0, angle: 90.0 }, // reverse arc
        Forward(15.0), // step
        Arc { radius: 5.0, angle: 90.0 },
        Forward(25.0), // east side
        Arc { radius: 5.0, angle: 90.0 },
        Forward(70.0), // north side
        Arc { radius: 5.0, angle: 90.0 },
        Forward(30.0), // west side
        Arc { radius: 5.0, angle: 90.0 },
    ];
    let block = plan_prism(m, 0.0, &plan, 10.0)?;
    // the rim over the middle of the north side: the round runs on round the whole chain
    let rim = edge_through(m, block, p(40.0, 40.0, 10.0))?;
    Ok(blend::fillet(m, block, &[rim], 2.0)?)
}
