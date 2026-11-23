//! A 30-tooth bevel gear's blank with one tooth space cut by a straight-sided tooth of its 15-tooth
//! pinion, the shafts square and meeting at O: the pinion turns by t about the x axis while the
//! gear turns by −t/2 about the z axis, t from −60° to 60°.
//!
//! The blank is its meridian quadrilateral revolved; the tooth a prism kept behind its four flank
//! planes through O. The tooth's turn seen from the gear's (`Trajectory::Relative`) is no constant
//! twist and keeps no plane, and the tooth is all planes and lines, so the kernel's sweep cut from
//! the blank (`swept::closed::cut_from`) takes the contact's road under any motion
//! (`swept::general`).
//!
//! Refused by the kernel at run time, before the trim: a wall between two of the contact's events
//! cannot be fitted through its stations, which stand too close to tell apart (`family::Stations`,
//! "a face family's stations stand on one another", `Unbuilt::Failed`). It is refused alike with
//! the sweep built closed alone (`swept::closed::closed`) and over the roll's first half alone.
use crate::gears::{bevel_blank, bevel_tooth, in_mesh, turn};
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;

/// How closely the tooth is meshed for the sweep's membership reads (its faces are flat).
const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let (blank, tooth) = (bevel_blank()?, bevel_tooth()?);
    let path = in_mesh(turn([0.; 3], [1., 0., 0.], 1.), -0.5);
    let roll = [-60f64.to_radians(), 60f64.to_radians()];
    cut_from(&blank, &tooth, &path, roll, BAR, tolerance(blank.size()))
        .map_err(|why| format!("the tooth's sweep not cut: {why:?}"))
}
