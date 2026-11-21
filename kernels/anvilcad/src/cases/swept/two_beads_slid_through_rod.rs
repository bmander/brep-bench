//! A rod of radius 5 on the z axis from z = 0 to 20 grooved by a wheel of two beads on one axis along y through (11.5, 0, −9), tori of middle circle 6 with tubes 2 and 1.5 (the second 1.5 along the axis), slid along z past it, from
//! below the rod to above it.
use anvil_brep::{Solid, extrude};

use crate::shared::{cylinder, e, frame, region, rod, stadium, subtract, torus, unite_all};

/// What a torus whose axis runs along y through (11.5, `y`, −9) (middle circle 6, tube `r`)
/// fills as it slides 38 along z: the tube of radius `r` about the stadium its middle circle
/// sweeps (its slab, the torus at each end, and a rod along each straight side).
fn slid(y: f64, r: f64) -> Result<Solid, String> {
    // anvil-brep has no sweep of a solid; a torus slid in its own plane is that tube
    let plane = frame([0., y, 0.], [0., 1., 0.], [1., 0., 0.])?;
    // in the plane y = const, its sketch x along world x and its sketch y along world −z
    let stadium = region(&[stadium((11.5, -29.), (11.5, 9.), 6.)])?;
    let slab = extrude::extrude(&plane, &stadium, -r, r).map_err(e)?;
    let wheel = |z: f64| torus(&frame([11.5, y, z], [0., 1., 0.], [1., 0., 0.])?, 6., r);
    // each joined to what is built so far, which it overlaps
    let pieces = [wheel(-9.), wheel(29.), rod([5.5, y, -9.], [5.5, y, 29.], r), rod([17.5, y, -9.], [17.5, y, 29.], r)];
    unite_all(slab, pieces)
}

pub fn build() -> Result<Solid, String> {
    let rod_ = cylinder(&anvil_brep::Frame::world(), 5., 0., 20.)?;
    // the sweep of the two beads' union is the union of their sweeps
    subtract(&subtract(&rod_, &slid(0., 2.)?)?, &slid(1.5, 1.5)?)
}
