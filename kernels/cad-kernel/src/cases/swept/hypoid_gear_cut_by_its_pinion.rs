//! The straight bevel case's blank with one tooth space cut by its tooth turned about the z axis by
//! φ (sin φ = 24/(75 − 5√5)), carried by a pinion whose axis is the line through (0, 12, 0) along
//! x: the pinion turns by t about it while the gear turns by −7t/15 about the z axis, t from −70°
//! to 70°.
//!
//! As in the straight bevel case, the kernel's arrangement (`swept::closed::cut_from`) takes the
//! tooth's turn seen from the gear's, no constant twist, only for a convex solid of revolution, and
//! refuses this polyhedral tooth by what it is.
use crate::gears::{bevel_blank, bevel_tooth, in_mesh, turn};
use cad_kernel::geom::{tolerance, Rigid};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;

const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let phi = (24. / (75. - 5. * 5f64.sqrt())).asin();
    let blank = bevel_blank()?;
    let tooth = bevel_tooth()?.moved(&Rigid::turn([0.; 3], [0., 0., 1.], phi));
    let path = in_mesh(turn([0., 12., 0.], [1., 0., 0.], 1.), -7. / 15.);
    let roll = [-70f64.to_radians(), 70f64.to_radians()];
    cut_from(&blank, &tooth, &path, roll, BAR, tolerance(blank.size()))
        .map_err(|why| format!("the tooth's sweep not cut: {why:?}"))
}
