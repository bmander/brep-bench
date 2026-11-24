use crate::solids::*;

pub fn build() -> R<Body> {
    let bar = block((-40.0, -8.0, -5.0), (40.0, 8.0, 5.0))?;
    // the eye about the y axis, its ends flush with the bar's sides
    let eye = solidcraft_kernel::cylinder(v3(0.0, -8.0, 0.0), Vec3::Y, 14.0, 16.0)?;
    let joined = union(&bar, &eye)?;
    // the four lines where the eye's side meets the bar's top and bottom, at their middles
    let x = 171f64.sqrt();
    let edges = [v3(x, 0.0, 5.0), v3(-x, 0.0, 5.0), v3(x, 0.0, -5.0), v3(-x, 0.0, -5.0)];
    solidcraft_kernel::fillet(&joined, &edges, 3.0)
}
