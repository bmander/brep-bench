//! A rod, the cylinder of radius 5 about z from z = 0 to 20, grooved along its length by a wheel
//! of two beads slid 40 along z: tori on one axis along y, the first centred at (11.5, 0, −9) of
//! radii 6 and 2, the second at (11.5, 1.5, −9) of radii 6 and 1.5, united.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, Profile, ProfileEdge};
use cad_kernel::geom::{Frame, V};
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, PI};

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
/// Each bead: its centre, the radius of the circle through its tube's centres, and its tube's.
const BEADS: [(V, f64, f64); 2] = [([11.5, 0., -9.], 6., 2.), ([11.5, 1.5, -9.], 6., 1.5)];
const SLIDE: f64 = 40.;

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
    // cad-kernel has no sweep of a solid, so the wheel's sweep is built by elementary geometry.
    // What a union passes through is what each of its parts passes through, so the wheel's sweep
    // is the two beads' sweeps united. Each bead is a solid of revolution slid along a line in its
    // own mid-plane: a point at distance ρ from the axis and h from the bead's mid-plane is inside
    // it where (ρ − R)² + h² ≤ r², and as it slides along z every ρ from |x − 11.5| (level with
    // the centre) up to as far as the slide reaches comes by, so a point is passed through just
    // where |x − 11.5| ≤ R + √(r² − h²): the stadium of the tube's two discs in the bead's
    // mid-plane seen along z, about (11.5 ∓ R, y), and the band between. Each slide is far longer
    // than the bead is across, so its shadow is filled from the level of the starting centre to
    // the last; beyond those levels the sweep is each bead, its hole filled, at each end, wholly
    // below and above the rod, which it does not touch.
    let [first, second] = BEADS.map(|(centre, major, minor)| {
        let shadow = stadium(centre[0] - major, centre[0] + major, centre[1], minor, centre[2]);
        prism(&shadow, 0., SLIDE)
    });
    let swept = combined(&first?, &second?, Op::Union, 0.)?;
    Ok(combined(&rod, &swept, Op::Cut, 0.)?)
}
