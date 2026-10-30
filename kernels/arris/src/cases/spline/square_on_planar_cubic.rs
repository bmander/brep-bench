//! A bar of square section 20 × 20 swept square to the planar cubic Bézier curve through the
//! control points `CONTROL`, two sides of the section horizontal and two upright, its ends flat.
use crate::Refusal;
use arris::geom::{FrameLaw, NurbsCurve, Path, PathSegment, Profile, ProfileLoop, ProfileSegment};
use arris::math::{Control, Frame, Point2, Point3, Vec3};
use arris::ops::{sweep, OpError};
use arris::topo::{Body, Model};

const CONTROL: [[f64; 3]; 4] =
    [[0.0, 0.0, 0.0], [16.0, 12.0, 0.0], [18.0, 26.0, 0.0], [12.0, 34.0, 0.0]];
const HALF: f64 = 10.0;

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let points: Vec<Point3> = CONTROL.iter().map(|&[x, y, z]| Point3::new(x, y, z)).collect();
    let (start, tangent) = (points[0], points[1] - points[0]);
    let bezier = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let curve = NurbsCurve::new(3, bezier, points, vec![1.0; 4]).map_err(|e| e.to_string())?;
    let path = Path { start, segments: vec![PathSegment::Nurbs(curve)] };
    // the square about the curve's start, square to it there: its u up, its v level
    let p = Point2::new;
    let section = Profile {
        plane: Frame::new(start, tangent, Vec3::z()).map_err(OpError::from)?,
        outer: ProfileLoop::Path {
            start: p(HALF, HALF),
            segments: vec![
                ProfileSegment::LineTo(p(-HALF, HALF)),
                ProfileSegment::LineTo(p(-HALF, -HALF)),
                ProfileSegment::LineTo(p(HALF, -HALF)),
                ProfileSegment::LineTo(p(HALF, HALF)),
            ],
        },
        holes: vec![],
    };
    // the binormal held up, so the section's sides stay horizontal and upright along the curve
    let law = FrameLaw::FixedBinormal(Vec3::z_axis());
    let (bar, _) = sweep(m, &section, &path, law, &Control::NONE)?;
    Ok(bar)
}
