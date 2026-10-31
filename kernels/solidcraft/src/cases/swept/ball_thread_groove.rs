//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a ball of
//! radius 1 whose centre runs along the helix of radius 5 about the z axis from (5, 0, −2),
//! right-handed and rising 4 a turn, for six turns.
//!
//! solidcraft-kernel has no sweep of a solid, so the ball's sweep is built by elementary geometry:
//! every point within 1 of the helix, which is the ball's great disc square to the helix carried
//! along it (the helix turns no tighter than radius 5.08, so the tube does not fold), with the
//! ball at each end. The screw that carries the helix along itself keeps the disc square to it,
//! so the tube is the kernel's helical sweep of the disc (`sweep_helix`), whose turns, 4 apart,
//! clear each other. The end balls, and the tube's end discs, lie wholly below z = −1 and above
//! z = 21, clear of the rod, so the tube is what the sweep takes from the rod.
use crate::solids::*;

pub fn build() -> R<Body> {
    let rod = zcyl(0.0, 0.0, 5.0, 0.0, 20.0)?;
    let start = v3(5.0, 0.0, -2.0);
    // the helix's direction where it starts: round the axis 5 a radian, up 4 a turn
    let along = v3(0.0, 5.0, 4.0 / std::f64::consts::TAU);
    let along = along * (1.0 / along.len());
    // the disc's plane: square to the helix, its sketch x out from the axis
    let plane = Plane { origin: start, x: Vec3::X, y: along.cross(Vec3::X) };
    // six turns about the z axis through the origin, rising 4 a turn
    let section = disc((0.0, 0.0), 1.0);
    let tube = solidcraft_kernel::sweep_helix(&plane, &section, Vec3::ZERO, Vec3::Z, 4.0, 6.0)?;
    cut(&rod, &tube)
}
