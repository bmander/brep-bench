//! A ball of radius 10 at the origin joined to a post of radius 4 on z from z = 0 to 15, the circle
//! where the post leaves the ball (radius 4 at z = √84) rounded to radius 2, adding material.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{ball_section, disc, prism, revolve};
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, Frame};
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const BALL_RADIUS: f64 = 10.;
const POST_RADIUS: f64 = 4.;
const POST_TOP: f64 = 15.;
const ROUND: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    let (centre, up) = ([0., 0., 0.], [0., 0., 1.]);
    // workaround: no sphere primitive
    // the ball: its half disc in the xz plane turned once about z
    let xz = Frame::new(centre, [0., -1., 0.], [1., 0., 0.]);
    let ball = revolve(&ball_section(&xz, BALL_RADIUS), centre, xz.y, TAU)?;
    let post = prism(&disc(centre, up, POST_RADIUS), 0., POST_TOP)?;
    let part = combined(&ball, &post, Op::Union, 0.)?;
    // the edge is picked by a point on it, where the post leaves the sphere; the kernel rolls the
    // ball between the post's side and the sphere round the loop
    let leaves = (BALL_RADIUS * BALL_RADIUS - POST_RADIUS * POST_RADIUS).sqrt();
    let round = roll(&part, [0., POST_RADIUS, leaves], ROUND, tolerance(part.size()))?;
    // a concave edge's round fills its piece in
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(&part, &round.piece, op, 0.)?)
}
