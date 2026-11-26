//! A block from (−20, −20, 0) to (20, 20, 8) with an L-shaped pocket cut 5 deep into its top: the
//! polygon through (−12, −12), (12, −12), (12, 0), (0, 0), (0, 12), (−12, 12), its five convex
//! corners rounded to radius 1 and its re-entrant corner to 3 about (3, 3), drawn with kentron's
//! pen on the floor z = 3 and extruded up past the top. The rim of the pocket's floor, twelve
//! edges in one tangent chain, rounded to radius 1, the convex corners' own radius.
use crate::bench::{edge_through, p};
use crate::shared::fillet::plan_prism;
use crate::Refusal;
use kentron::blend;
use kentron::boolean;
use kentron::primitives::Cuboid;
use kentron::sketch::Step::{Arc, Forward, Start};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(-20.0, -20.0, 0.0), p(20.0, 20.0, 8.0)).build(m)?;
    let outline = [
        Start(-11.0, -12.0),
        Forward(22.0), Arc { radius: 1.0, angle: 90.0 },
        Forward(10.0), Arc { radius: 1.0, angle: 90.0 },
        Forward(8.0), Arc { radius: -3.0, angle: 90.0 }, // the re-entrant corner, about (3, 3)
        Forward(8.0), Arc { radius: 1.0, angle: 90.0 },
        Forward(10.0), Arc { radius: 1.0, angle: 90.0 },
        Forward(22.0), Arc { radius: 1.0, angle: 90.0 },
    ];
    let pocket = plan_prism(m, 3.0, &outline, 10.0)?;
    let block = boolean::difference(m, block, pocket)?;
    let rim = edge_through(m, block, p(0.0, -12.0, 3.0))?;
    Ok(blend::fillet(m, block, &[rim], 1.0)?)
}
