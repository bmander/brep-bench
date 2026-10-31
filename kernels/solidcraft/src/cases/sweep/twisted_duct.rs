use crate::solids::*;
use solidcraft_kernel::loft;

/// A ruled loft between the two sections, each drawn in x and z on its plane, its corners in
/// the order A, B, C, D: between two sections the kernel rules each wall straight from one
/// section's side to the other's.
pub fn build() -> R<Body> {
    let at = |y: f64| Plane { origin: v3(0.0, y, 0.0), x: Vec3::X, y: Vec3::Z };
    let square = [(10.0, -10.0), (-10.0, -10.0), (-10.0, 10.0), (10.0, 10.0)];
    let diamond = [(0.0, -12.0), (-12.0, 0.0), (0.0, 12.0), (12.0, 0.0)];
    loft(&[(at(0.0), polygon(&square).outer), (at(40.0), polygon(&diamond).outer)])
}
