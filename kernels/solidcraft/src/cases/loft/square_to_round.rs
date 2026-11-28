use crate::solids::*;
use solidcore_record::Seg2;
use solidcraft_kernel::{patch_edges, patch_region, revolve_sheet, stitch, transform_matrix};
use std::f64::consts::FRAC_PI_2;

/// The square's half side, the circle's radius and the circle's height.
#[derive(Clone, Copy)]
pub struct Sizes {
    pub half: f64,
    pub radius: f64,
    pub height: f64,
}

/// The flat face through the points `pts`.
fn flat(pts: &[Vec3]) -> R<Body> {
    let plane = Plane::new(pts[0], pts[1] - pts[0], pts[2] - pts[0])
        .ok_or_else(|| KernelError::Invalid("a face in no plane".into()))?;
    let local: Vec<Vec2> = pts.iter().map(|p| plane.to_local(*p)).collect();
    patch_region(&plane, &Region2 { outer: Loop2::polygon(&local), holes: vec![] })
}

/// The piece of the oblique cone with its apex at the square's corner `c` through the quarter
/// of the circle from the angle `a` (radians about the z axis) a quarter turn on. The kernel
/// has no oblique cone; it is the right cone with its apex at `c` through the circle of the
/// same radius right above it, revolved from a ruling, then sheared, the floor staying put,
/// until that circle is the top's: (x, y, z) to (x − c.x·z/h, y − c.y·z/h, z), h the height,
/// which moves each height's circle across without turning it, so the quarter stays the quarter.
fn cone(Sizes { radius, height, .. }: Sizes, c: Vec3, a: f64) -> R<Body> {
    let (s, k) = a.sin_cos();
    // drawn in the vertical plane through the corner toward the quarter's start, its y down, so
    // that the revolve, which turns toward the plane's normal, turns from the start to the end
    let plane = Plane { origin: c, x: v3(k, s, 0.0), y: Vec3::Z * -1.0 };
    let ruling = vec![Seg2::Line { a: Vec2::ZERO, b: v2(radius, -height) }];
    let right = revolve_sheet(&plane, &[ruling], Vec2::ZERO, Vec2::Y, 0.0, FRAC_PI_2)?;
    // column by column
    let shear = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-c.x / height, -c.y / height, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    transform_matrix(&right, shear)
}

/// SolidCraft's loft pairs its sections' segments one to one, so it cannot join a side of the
/// square to a point of the circle, nor a corner to an arc. The transition is stitched from its
/// faces instead, as SolidCraft's surface tools stitch a closed set of sheets into a solid: the
/// floor, the round top, a flat triangle from each side to the point of the circle nearest it,
/// and a piece of an oblique cone from each corner to the quarter circle facing it.
pub fn build() -> R<Body> {
    transition(Sizes { half: 20.0, radius: 15.0, height: 30.0 })
}

/// The transition from the square of half side `half` on z = 0 to the circle of `radius` on
/// z = `height`, both about the z axis.
pub fn transition(sizes: Sizes) -> R<Body> {
    let Sizes { half, radius, height } = sizes;
    let mut sheets = vec![patch_region(&Plane::XY, &rect(-half, -half, half, half))?];
    let mut rim = Vec::new();
    for k in 0..4 {
        // corner k is (half, −half) turned 90°·k about the z axis, and its quarter circle runs
        // from (0, −radius) turned as far
        let (s, c) = (FRAC_PI_2 * k as f64).sin_cos();
        let turned = |x: f64, y: f64, z: f64| v3(x * c - y * s, x * s + y * c, z);
        let (corner, next) = (turned(half, -half, 0.0), turned(half, half, 0.0));
        let a = FRAC_PI_2 * (k as f64 - 1.0);
        sheets.push(cone(sizes, corner, a)?);
        // the side from this corner to the next rises to the point of the circle nearest it
        sheets.push(flat(&[corner, next, turned(radius, 0.0, height)])?);
        // the revolve makes the quarter as two eighths: a point inside each
        for m in [a + FRAC_PI_2 / 4.0, a + 3.0 * FRAC_PI_2 / 4.0] {
            rim.push(v3(radius * m.cos(), radius * m.sin(), height));
        }
    }
    let walls = stitch(&sheets.iter().collect::<Vec<_>>(), 0.0)?;
    // the top is the patch on the walls' own rim, so that it shares their edges
    let top = patch_edges(&walls, &rim)?;
    stitch(&[&walls, &top], 0.0)
}
