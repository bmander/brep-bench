use crate::solids::*;
use solidcraft_geom::Seg2;
use solidcraft_kernel::{patch_edges, patch_region, revolve_sheet, stitch, transform_matrix};
use std::f64::consts::FRAC_PI_2;

const HALF: f64 = 20.0;
const RADIUS: f64 = 15.0;
const HEIGHT: f64 = 30.0;

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
/// until that circle is the top's: (x, y, z) to (x − c.x·z/30, y − c.y·z/30, z), which moves
/// each height's circle across without turning it, so the quarter stays the quarter.
fn cone(c: Vec3, a: f64) -> R<Body> {
    let (s, k) = a.sin_cos();
    // drawn in the vertical plane through the corner toward the quarter's start, its y down, so
    // that the revolve, which turns toward the plane's normal, turns from the start to the end
    let plane = Plane { origin: c, x: v3(k, s, 0.0), y: Vec3::Z * -1.0 };
    let ruling = vec![Seg2::Line { a: Vec2::ZERO, b: v2(RADIUS, -HEIGHT) }];
    let right = revolve_sheet(&plane, &[ruling], Vec2::ZERO, Vec2::Y, 0.0, FRAC_PI_2)?;
    // column by column
    let shear = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-c.x / HEIGHT, -c.y / HEIGHT, 1.0, 0.0],
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
    let mut sheets = vec![patch_region(&Plane::XY, &rect(-HALF, -HALF, HALF, HALF))?];
    let mut rim = Vec::new();
    for k in 0..4 {
        // corner k is (20, −20) turned 90°·k about the z axis, and its quarter circle runs
        // from (0, −15) turned as far
        let (s, c) = (FRAC_PI_2 * k as f64).sin_cos();
        let turned = |x: f64, y: f64, z: f64| v3(x * c - y * s, x * s + y * c, z);
        let (corner, next) = (turned(HALF, -HALF, 0.0), turned(HALF, HALF, 0.0));
        let a = FRAC_PI_2 * (k as f64 - 1.0);
        sheets.push(cone(corner, a)?);
        // the side from this corner to the next rises to the point of the circle nearest it
        sheets.push(flat(&[corner, next, turned(RADIUS, 0.0, HEIGHT)])?);
        // the revolve makes the quarter as two eighths: a point inside each
        for m in [a + FRAC_PI_2 / 4.0, a + 3.0 * FRAC_PI_2 / 4.0] {
            rim.push(v3(RADIUS * m.cos(), RADIUS * m.sin(), HEIGHT));
        }
    }
    let walls = stitch(&sheets.iter().collect::<Vec<_>>(), 0.0)?;
    // the top is the patch on the walls' own rim, so that it shares their edges
    let top = patch_edges(&walls, &rim)?;
    stitch(&[&walls, &top], 0.0)
}
