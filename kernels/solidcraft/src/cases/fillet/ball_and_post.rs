use crate::solids::*;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 10.0)?;
    let post = zcyl(0.0, 0.0, 4.0, 0.0, 15.0)?;
    let joined = union(&ball, &post)?;
    // a point on the circle where the post leaves the ball: radius 4 at z = √84
    solidcraft_kernel::fillet(&joined, &[v3(0.0, 4.0, 84f64.sqrt())], 2.0)
}
