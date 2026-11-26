//! The prolate spheroid (x² + y²)/9 + z²/36 ≤ 1, the half ellipse x²/9 + z²/36 ≤ 1, x ≥ 0, of the
//! xz plane turned a whole turn about the z axis, its long axis, hollowed to a wall 1 thick with no
//! opening, as `shell/sphere-hollowed` hollows its ball: springrep's own shell.
use std::f64::consts::{FRAC_PI_2, PI};

use springrep::edit::Wall;
use springrep::math::Vec2;
use springrep::profile::{NamedSegment, Segment};
use springrep::topo::Brep;

use crate::shared::shell::{line, shelled, turned_whole};

pub fn build() -> Result<Brep, String> {
    // the major axis, 6, along the meridian's y (world z); its minor, 3, toward the meridian's −x,
    // so from t = π to 2π the arc runs on x ≥ 0 from the bottom pole (0, −6) to the top (0, 6)
    let arc = Segment::Ellipse {
        center: Vec2::ZERO,
        rotation: FRAC_PI_2,
        major: 6.,
        minor: 3.,
        start: PI,
        sweep: PI,
    };
    let spheroid = turned_whole(
        vec![NamedSegment::new(arc, "skin"), line([0., 6.], [0., -6.], "axis")],
        "spheroid",
    )?;
    shelled(&spheroid, &[], 1., Wall::Inward, "shell")
}
