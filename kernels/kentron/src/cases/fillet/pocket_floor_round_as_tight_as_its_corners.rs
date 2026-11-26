//! A block from (−40, −30, 0) to (40, 30, 16) with a pocket cut 10 deep into its top: the
//! rectangle from (−25, −15) to (25, 15) with its corners rounded to radius 4, drawn with
//! kentron's pen on the floor z = 6 and extruded up past the top. The rim of the pocket's floor,
//! eight edges in one tangent chain, rounded to radius 4, the corners' own radius.
use crate::bench::{edge_through, p};
use crate::shared::fillet::plan_prism;
use crate::Refusal;
use kentron::blend;
use kentron::boolean;
use kentron::primitives::Cuboid;
use kentron::sketch::Step::{Arc, Forward, Start};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(-40.0, -30.0, 0.0), p(40.0, 30.0, 16.0)).build(m)?;
    let outline = [
        Start(-21.0, -15.0),
        Forward(42.0), Arc { radius: 4.0, angle: 90.0 },
        Forward(22.0), Arc { radius: 4.0, angle: 90.0 },
        Forward(42.0), Arc { radius: 4.0, angle: 90.0 },
        Forward(22.0), Arc { radius: 4.0, angle: 90.0 },
    ];
    let pocket = plan_prism(m, 6.0, &outline, 20.0)?;
    let block = boolean::difference(m, block, pocket)?;
    let rim = edge_through(m, block, p(0.0, -15.0, 6.0))?;
    Ok(blend::fillet(m, block, &[rim], 4.0)?)
}
