//! A pipe of radius 8 swept square to the cubic Bézier curve, not lying in a plane, through the
//! control points `CONTROL`, its ends flat discs square to the curve.
use crate::Refusal;
use arris::geom::{FrameLaw, NurbsCurve, Path, PathSegment, Profile, ProfileLoop};
use arris::math::{Control, Frame, Point2, Point3, Vec3};
use arris::ops::{sweep, OpError};
use arris::topo::{Body, Model};

const CONTROL: [[f64; 3]; 4] =
    [[0.0, 0.0, 0.0], [12.0, 16.0, 0.0], [26.0, 8.0, 0.0], [24.0, 0.0, 16.0]];
const PIPE_RADIUS: f64 = 8.0;

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let points: Vec<Point3> = CONTROL.iter().map(|&[x, y, z]| Point3::new(x, y, z)).collect();
    let (start, tangent) = (points[0], points[1] - points[0]);
    let bezier = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let curve = NurbsCurve::new(3, bezier, points, vec![1.0; 4]).map_err(|e| e.to_string())?;
    let path = Path { start, segments: vec![PathSegment::Nurbs(curve)] };
    // the section square to the curve where it starts, its tangent there along the first leg; a
    // disc, so how the rotation-minimising frame turns it about the curve does not matter
    let section = Profile {
        plane: Frame::new(start, tangent, Vec3::z()).map_err(OpError::from)?,
        outer: ProfileLoop::Circle { center: Point2::origin(), radius: PIPE_RADIUS },
        holes: vec![],
    };
    let (pipe, _) = sweep(m, &section, &path, FrameLaw::RotationMinimizing, &Control::NONE)?;
    Ok(pipe)
}
