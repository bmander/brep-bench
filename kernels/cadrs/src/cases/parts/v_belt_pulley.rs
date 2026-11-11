//! A V-belt pulley turned about the z axis: a hub of radius 12 from z = −14 to z = 14, a rim of
//! radius 28 from z = −9 to z = 9 and a bore of radius 6 right through, one section on the Front
//! plane (sketch (x, y) the model's (x, z)) revolved a whole turn; less the belt groove, the
//! triangle with corners (22, 0), (29, −5) and (29, 5) revolved a whole turn, reaching 1 past the
//! rim's face.
use crate::bench::{cut, p3, revolve, V3};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = Axis::new(p3(0.0, 0.0, 0.0), V3::z());
    let section = Loop::polygon(&[
        [6.0, -14.0],
        [12.0, -14.0],
        [12.0, -9.0],
        [28.0, -9.0],
        [28.0, 9.0],
        [12.0, 9.0],
        [12.0, 14.0],
        [6.0, 14.0],
    ]);
    let pulley = revolve(k, &Profile::of(Plane::xz(0.0), vec![section]), z, None)?;
    let triangle = Loop::polygon(&[[22.0, 0.0], [29.0, -5.0], [29.0, 5.0]]);
    let groove = revolve(k, &Profile::of(Plane::xz(0.0), vec![triangle]), z, None)?;
    cut(k, pulley, groove)
}
