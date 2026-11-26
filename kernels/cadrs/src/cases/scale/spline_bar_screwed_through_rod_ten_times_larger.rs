//! A rod (radius 50, z from 0 to 200) threaded by a bar whose section, in the plane y = 0, is the
//! closed uniform periodic cubic B-spline on (35, 0), (45, −15), (80, −15), (120, −10), (120, 10),
//! (80, 15), (45, 15) in (x, z), the bar running from y = −80 to y = 80, screwed about the z axis
//! rising 60 a turn from −90° to 1350° (`swept/spline-bar-screwed-through-rod` ten times larger).
//!
//! The section is a periodic spline sketched on the plane y = 80 and extruded 160 toward −y; the
//! part of the bar within 55 of the axis is what reaches the rod; cadrs's cut by a swept solid.
use crate::bench::{cylinder, extrude, p3, v3};
use crate::shared::geometry::Section;
use crate::shared::swept::{cut_swept, near_axis, screw, z_axis};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Plane};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    // The plane y = 80, sketch x along +x and sketch y along +z (normal −y, the extrusion's way).
    let on = Plane::new(p3(0.0, 80.0, 0.0), v3(1.0, 0.0, 0.0), v3(0.0, -1.0, 0.0));
    // (The same loop started from (120, −10): its seam, where the periodic spline's first span
    // starts, then lies at its far end, out of the part near the axis.)
    let poles = [(120.0, -10.0), (120.0, 10.0), (80.0, 15.0), (45.0, 15.0), (35.0, 0.0), (45.0, -15.0), (80.0, -15.0)];
    let section = Section::on(on).periodic(3, &poles);
    let bar = extrude(k, &section.sketch(), 160.0)?;
    let bar = near_axis(k, bar, 55.0)?;
    let r = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 50.0, 200.0)?;
    cut_swept(k, r, bar, &screw(z_axis(), 60.0, -90.0, 1350.0))
}
