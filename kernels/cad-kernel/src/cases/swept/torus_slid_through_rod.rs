//! A rod, the cylinder of radius 5 about z from z = 0 to 20, grooved along its length by a torus
//! (axis along y, radii 6 and 2) slid without turning along z from centre (11.5, 0, −9) to
//! (11.5, 0, 29).
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, Profile, ProfileEdge};
use cad_kernel::geom::{Frame, V};
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, PI};

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
const CENTRE: V = [11.5, 0., -9.];
const MAJOR: f64 = 6.;
const MINOR: f64 = 2.;
const SLIDE: f64 = 38.;

/// The stadium in the plane through `z` square to z: the two discs of radius `r` about
/// (`x0`, `y`) and (`x1`, `y`) and the rectangle between them, as two lines and two half circles.
fn stadium(x0: f64, x1: f64, y: f64, r: f64, z: f64) -> Profile {
    let up = [0., 0., 1.];
    let half = |x: f64, from: f64| ProfileEdge::Arc {
        frame: Frame::new([x, y, z], up, [1., 0., 0.]),
        r,
        span: Some([from, from + PI]),
    };
    let edges = vec![
        ProfileEdge::Line { a: [x0, y - r, z], b: [x1, y - r, z] },
        half(x1, -FRAC_PI_2),
        ProfileEdge::Line { a: [x1, y + r, z], b: [x0, y + r, z] },
        half(x0, FRAC_PI_2),
    ];
    Profile { origin: [x0, y, z], normal: up, loops: vec![edges], names: Vec::new() }
}

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0., 0., 0.], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    // cad-kernel has no sweep of a solid, so the torus's sweep is built by elementary geometry. A
    // solid of revolution slid along a line in its own mid-plane passes over every point square
    // to the line of its shadow: a point at distance ρ from the torus's axis and y from its
    // mid-plane is inside it where (ρ − 6)² + y² ≤ 2², and as the torus slides along z every ρ
    // from |x − 11.5| (level with the centre) up to as far as the slide reaches comes by, so a
    // point is passed through just where |x − 11.5| ≤ 6 + √(2² − y²): the stadium of the tube's
    // two discs in the plane y = 0 seen along z, (5.5, 0) and (17.5, 0) of radius 2, and the band
    // between. The torus's slide is far longer than it is across, so that shadow is filled from
    // the level of the starting centre to the last; beyond those levels the sweep is the torus,
    // its hole filled, at each end, wholly below and above the rod, which it does not touch.
    let reach = |s: f64| CENTRE[0] + s * MAJOR;
    let shadow = stadium(reach(-1.), reach(1.), CENTRE[1], MINOR, CENTRE[2]);
    let swept = prism(&shadow, 0., SLIDE)?;
    Ok(combined(&rod, &swept, Op::Cut, 0.)?)
}
