//! A square plate, the box from (−50, −50, 0) to (50, 50, 20), joined to a round post standing
//! across its edge, the cylinder of radius 15 about the vertical line through (45, 0) from z = 0 to
//! z = 40, its overhanging foot flush with the plate's bottom. A hole of radius 6 drilled right
//! through along the post's axis, from the bottom face z = 0 to the post's top z = 40, and both its
//! rims chamfered 1.5 along each face at 45°: the lower one runs unbroken across the line x = 50,
//! where the plate's bottom and the post's foot meet in the one plane.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::fillet::flat_along;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const HALF: f64 = 50.;
const THICKNESS: f64 = 20.;
const AXIS: [f64; 2] = [45., 0.];
const POST_RADIUS: f64 = 15.;
const POST_HEIGHT: f64 = 40.;
const HOLE_RADIUS: f64 = 6.;
const CHAMFER: f64 = 1.5;

pub fn build() -> Result<Brep, String> {
    let plate = block([-HALF, -HALF, 0.], [HALF, HALF, THICKNESS])?;
    let [x, y] = AXIS;
    let post = prism(&disc([x, y, 0.], [0., 0., 1.], POST_RADIUS), 0., POST_HEIGHT)?;
    let part = combined(&plate, &post, Op::Union, 0.)?;
    // the drill runs past the bottom and the top, from z = −1 to z = 41, its wall named
    let mut section = disc([x, y, -1.], [0., 0., 1.], HOLE_RADIUS);
    section.names = vec![vec!["bore".into()]];
    let drill = prism(&section, 0., POST_HEIGHT + 2.)?;
    let mut part = combined(&part, &drill, Op::Cut, 0.)?;
    // each rim picked by a point on it, the lower one where it crosses into the overhang's foot
    // (x = 51) and the upper on the post's top; each is a circle about the axis the hole's wall and
    // the flat face share, so the kernel's flat is the 45° cone between the legs, exactly
    for at in [[x + HOLE_RADIUS, y, 0.], [x - HOLE_RADIUS, y, POST_HEIGHT]] {
        let piece = flat_along(&part, at, [CHAMFER, CHAMFER], None, tolerance(part.size()))?;
        // the rim is convex: its piece is taken away
        part = combined(&part, &piece, Op::Cut, 0.)?;
    }
    Ok(part)
}
