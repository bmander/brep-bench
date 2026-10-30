//! A truncated cone on the z axis, 6 high, of radius 5 at its base and 2 at its top.
use arris::geom::{Profile, ProfileLoop};
use arris::math::{Control, Frame, Point2, Point3, Vec3};
use arris::ops::{loft, LoftKind, OpError};
use arris::topo::{Body, Model};

const BASE_RADIUS: f64 = 5.0;
const TOP_RADIUS: f64 = 2.0;
const HEIGHT: f64 = 6.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let disc = |z: f64, radius: f64| -> Result<Profile, OpError> {
        Ok(Profile {
            plane: Frame::new(Point3::new(0.0, 0.0, z), Vec3::z(), Vec3::x())?,
            outer: ProfileLoop::Circle { center: Point2::origin(), radius },
            holes: vec![],
        })
    };
    // workaround: no cone primitive
    // arris has no cone primitive: a ruled loft between the two end circles is one, and arris
    // recognises a span between two coaxial circles as an exact cone
    let sections = [disc(0.0, BASE_RADIUS)?, disc(HEIGHT, TOP_RADIUS)?];
    let (frustum, _) = loft(m, &sections, LoftKind::Ruled, false, &Control::NONE)?;
    Ok(frustum)
}
