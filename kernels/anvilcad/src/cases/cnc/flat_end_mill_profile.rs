//! A plate from the origin to (120, 90, −15) with a groove milled round a cross (its corners
//! rounded to radius 6) by a flat end mill of radius 3 at depths 3, 6 and 9.
use anvil_brep::taper::offset_loop;
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{block, e, region, subtract};

/// The cross with each corner rounded to radius 6 by a quarter circle tangent to its sides.
fn cross() -> anvil_brep::Loop2 {
    use anvil_brep::Segment;
    use std::f64::consts::FRAC_PI_2;
    let corners = [
        (45., 10.), (75., 10.), (75., 30.), (100., 30.), (100., 60.), (75., 60.),
        (75., 80.), (45., 80.), (45., 60.), (20., 60.), (20., 30.), (45., 30.),
    ];
    let p: Vec<glam::DVec2> = corners.iter().map(|&(x, y)| glam::DVec2::new(x, y)).collect();
    let n = p.len();
    let mut segments = Vec::new();
    for i in 0..n {
        let (a, b, c) = (p[(i + n - 1) % n], p[i], p[(i + 1) % n]);
        let (din, dout) = ((b - a).normalize(), (c - b).normalize());
        // the arc starts 6 back from the corner, its center 6 in from both sides
        let from = b - din * 6.;
        let left = din.perp_dot(dout) > 0.;
        let center = if left { from + din.perp() * 6. } else { from - din.perp() * 6. };
        let start = (from - center).to_angle();
        let sweep = if left { FRAC_PI_2 } else { -FRAC_PI_2 };
        if let Some(last) = segments.last().map(|s: &Segment| s.end()) {
            segments.push(Segment::Line { a: last, b: from });
        }
        segments.push(Segment::Arc { center, radius: 6., start, sweep });
    }
    let (first, last) = (segments[0].start(), segments[segments.len() - 1].end());
    segments.push(Segment::Line { a: last, b: first });
    anvil_brep::Loop2::new(segments)
}

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no sweep of a solid. The mill, an upright cylinder of radius 3 sliding level
    // round a path that turns no tighter than 6, sweeps the band 3 either side of the path; its
    // three laps cut the same walls, down to its deepest at z = −9 (its plunges lie in the band)
    let path = cross();
    let groove = region(&[offset_loop(&path, 3.).map_err(e)?, offset_loop(&path, -3.).map_err(e)?])?;
    let cut = extrude::extrude(&Frame::world(), &groove, -9., 1.).map_err(e)?;
    subtract(&block([0., 0., -15.], [120., 90., 0.])?, &cut)
}
