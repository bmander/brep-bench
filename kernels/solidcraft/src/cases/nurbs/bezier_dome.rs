use crate::solids::*;
use solidcraft_geom::Seg2;
use solidcraft_geom::bspline::NurbsSurface;
use solidcraft_geom::tspline::TSpline;

/// The heights zᵢⱼ of the dome's control points, row i at x = 20i/3, column j at y = 20j/3.
pub const HEIGHTS: [[f64; 4]; 4] =
    [[4.0, 6.0, 7.0, 5.0], [6.0, 14.0, 12.0, 7.0], [5.0, 13.0, 16.0, 8.0], [3.0, 6.0, 8.0, 6.0]];

/// The bicubic Bézier patch: a tensor B-spline of degree 3 each way on the knots
/// (0, 0, 0, 0, 1, 1, 1, 1), its control points u (x) fastest.
pub fn patch() -> Result<NurbsSurface, String> {
    let knots = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let at = |k: usize| 20.0 * k as f64 / 3.0;
    let pts: Vec<Vec3> =
        (0..4).flat_map(|j| (0..4).map(move |i| v3(at(i), at(j), HEIGHTS[i][j]))).collect();
    NurbsSurface::new([3, 3], [knots.clone(), knots], &pts, &[1.0; 16]).map_err(|e| e.to_string())
}

/// The surface as a sheet: SolidCraft's NURBS sheet is a T-spline's (`tspline_sheet`), and a
/// T-mesh without T-junctions is the tensor surface itself, one face bounded by its four
/// boundary curves.
pub fn sheet(s: &NurbsSurface) -> Result<Body, crate::Refusal> {
    let ts = TSpline::from_nurbs(s).map_err(|e| e.to_string())?;
    Ok(solidcraft_kernel::tspline_sheet(&ts)?)
}

/// The side in the upright plane through `origin` along `along`, 20 long: the flat region
/// between the floor and the curve the dome's edge draws there, `edge` (in the side's own x and
/// z, running along it from 0 to 20).
pub fn side(origin: Vec3, along: Vec3, edge: &[Seg2]) -> R<Body> {
    let plane = Plane { origin, x: along, y: Vec3::Z };
    let (start, end) = (edge[0].start(), edge[edge.len() - 1].end());
    let mut segs = vec![
        Seg2::Line { a: v2(0.0, 0.0), b: v2(20.0, 0.0) },
        Seg2::Line { a: v2(20.0, 0.0), b: end },
    ];
    segs.extend(edge.iter().rev().map(Seg2::reversed));
    segs.push(Seg2::Line { a: start, b: v2(0.0, 0.0) });
    solidcraft_kernel::patch_region(&plane, &Region2 { outer: Loop2 { segs }, holes: vec![] })
}

/// The block on the square 0 ≤ x, y ≤ 20 under the sheets `top`: the sheets, the floor and the
/// four sides, each a planar patch of a sketch region, stitched into one solid along the edges
/// they share. `edges` are the curves the sheets' edges draw in the sides y = 0, x = 20, y = 20
/// and x = 0, each in its side's x (along +x or +y) and z, one segment for each sheet's edge.
pub fn block_under(top: &[Body], edges: [Vec<Seg2>; 4]) -> Result<Body, crate::Refusal> {
    let floor = solidcraft_kernel::patch_region(&Plane::XY, &rect(0.0, 0.0, 20.0, 20.0))?;
    let [south, east, north, west] = edges;
    let sides = [
        side(Vec3::ZERO, Vec3::X, &south)?,
        side(v3(20.0, 0.0, 0.0), Vec3::Y, &east)?,
        side(v3(0.0, 20.0, 0.0), Vec3::X, &north)?,
        side(Vec3::ZERO, Vec3::Y, &west)?,
    ];
    let faces: Vec<&Body> = top.iter().chain([&floor]).chain(&sides).collect();
    Ok(solidcraft_kernel::stitch(&faces, 1e-6)?)
}

/// The block under the dome. Each side's top edge is the cubic Bézier over one boundary row or
/// column of the control net: its control points stand at 0, 20/3, 40/3 and 20 along the side.
pub fn block() -> Result<Body, crate::Refusal> {
    let z = HEIGHTS;
    let cubic = |h: [f64; 4]| {
        let at = |k: usize| v2(20.0 * k as f64 / 3.0, h[k]);
        vec![Seg2::Cubic { p0: at(0), p1: at(1), p2: at(2), p3: at(3) }]
    };
    let edges = [
        cubic([z[0][0], z[1][0], z[2][0], z[3][0]]),
        cubic(z[3]),
        cubic([z[0][3], z[1][3], z[2][3], z[3][3]]),
        cubic(z[0]),
    ];
    block_under(&[sheet(&patch()?)?], edges)
}

pub fn build() -> Result<Body, crate::Refusal> {
    block()
}
