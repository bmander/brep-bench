//! The two sectors of `singular/sectors-sharing-an-apex`, ten times larger: two spherical sectors
//! of radius 100 and half-angle 45°, apex at the origin, one about z and one about z turned 60°
//! about y, the solid common to both.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::{Frame, Rigid};
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_4, TAU};

const RADIUS: f64 = 100.;
const HALF_ANGLE: f64 = FRAC_PI_4;
const TURN_DEGREES: f64 = 60.;

/// The sector about +z: its section in the xz plane — the ruling from the apex out to the rim, the
/// arc of the sphere up to the axis, and the axis back down — turned a whole turn about z.
fn sector() -> Result<Brep, String> {
    let apex = [0., 0., 0.];
    let rim = [RADIUS * HALF_ANGLE.sin(), 0., RADIUS * HALF_ANGLE.cos()];
    let top = [0., 0., RADIUS];
    // the arc about the apex, its angle measured from +z towards +x
    let frame = Frame::new(apex, [0., -1., 0.], [0., 0., 1.]);
    let edges = vec![
        ProfileEdge::Line { a: apex, b: rim },
        ProfileEdge::Arc { frame, r: RADIUS, span: Some([-HALF_ANGLE, 0.]) },
        ProfileEdge::Line { a: top, b: apex },
    ];
    let section = Profile { origin: apex, normal: [0., -1., 0.], loops: vec![edges], names: Vec::new() };
    revolve(&section, apex, [0., 0., 1.], TAU)
}

pub fn build() -> Result<Brep, String> {
    let first = sector()?;
    // the second: the first turned about y, its axis towards +x
    let second = first.moved(&Rigid::turn([0., 0., 0.], [0., 1., 0.], TURN_DEGREES.to_radians()));
    Ok(combined(&first, &second, Op::Common, 0.)?)
}
