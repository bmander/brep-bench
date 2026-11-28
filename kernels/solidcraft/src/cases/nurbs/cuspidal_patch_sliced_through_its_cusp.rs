//! The block under the part 11/20 ≤ u ≤ 1 of the cuspidal Bézier patch, of degree 3 in u and 1 in
//! v, control points (xᵢ, j, zᵢ + j/10), sliced by the plane z = 11y/10 through the patch's cusp.
//!
//! As the case allows, the top face is built on the patch cut down to that part (the same face):
//! the geometry crate's de Casteljau subdivision of the patch's rows at u = 11/20 (`piece`).
use super::bezier_dome::sheet;
use crate::Refusal;
use crate::solids::*;
use solidcore_record::Seg2;
use solidcraft_geom::bspline::{HPoint, NurbsSurface, bezier_segment};

/// The control net of the piece of a Bézier patch over u ≥ `a`, v ≥ `b`, `net[i][j]` the control
/// point i along u and j along v: each row along u, then each along v, cut down by the geometry
/// crate's de Casteljau subdivision (`bezier_segment`).
pub fn piece(net: &[Vec<Vec3>], a: f64, b: f64) -> Result<Vec<Vec<Vec3>>, Refusal> {
    let cut = |row: Vec<Vec3>, from: f64| -> Result<Vec<Vec3>, Refusal> {
        let h: Vec<HPoint> = row.iter().map(|&p| HPoint::weighted(p, 1.0)).collect();
        let part = bezier_segment(&h, from, 1.0).map_err(|e| e.to_string())?;
        Ok(part.iter().map(|q| v3(q.x / q.w, q.y / q.w, q.z / q.w)).collect())
    };
    let (n, m) = (net.len(), net[0].len());
    // the rows along u, column j of the net each
    let mut columns = Vec::new();
    for j in 0..m {
        let row: Vec<Vec3> = (0..n).map(|i| net[i][j]).collect();
        columns.push(if a > 0.0 { cut(row, a)? } else { row });
    }
    // then along v, row i each
    let mut out = Vec::new();
    for i in 0..n {
        let row: Vec<Vec3> = (0..m).map(|j| columns[j][i]).collect();
        out.push(if b > 0.0 { cut(row, b)? } else { row });
    }
    Ok(out)
}

/// The edge a Bézier patch's boundary row `row` draws in its upright side, in the side's own x
/// (`along` of each point) and z.
fn edge(row: &[Vec3], along: impl Fn(&Vec3) -> f64) -> Result<Seg2, Refusal> {
    let p: Vec<Vec2> = row.iter().map(|q| v2(along(q), q.z)).collect();
    match p[..] {
        [a, b] => Ok(Seg2::Line { a, b }),
        [p0, p1, p2, p3] => Ok(Seg2::Cubic { p0, p1, p2, p3 }),
        _ => Err(format!("a side's edge of degree {} here", p.len() - 1).into()),
    }
}

/// The flat side in the upright plane through `origin` along `along`, between the floor and the
/// patch's edge `top`, which runs along it from 0 to `length`.
fn side(origin: Vec3, along: Vec3, length: f64, top: Seg2) -> R<Body> {
    let plane = Plane { origin, x: along, y: Vec3::Z };
    let segs = vec![
        Seg2::Line { a: v2(0.0, 0.0), b: v2(length, 0.0) },
        Seg2::Line { a: v2(length, 0.0), b: top.end() },
        top.reversed(),
        Seg2::Line { a: top.start(), b: v2(0.0, 0.0) },
    ];
    solidcraft_kernel::patch_region(&plane, &Region2 { outer: Loop2 { segs }, holes: vec![] })
}

/// The block on the floor z = 0 under a Bézier patch of odd degrees (`net[i][j]`, i along u)
/// that is the graph of a height over the rectangle of its corner control points, x along u and
/// y along v: the patch as a sheet, the floor and the four upright sides, each a planar patch of
/// a sketch region, stitched into one solid along the edges they share.
pub fn block_under_patch(net: &[Vec<Vec3>]) -> Result<Body, Refusal> {
    let (n, m) = (net.len() - 1, net[0].len() - 1);
    let bezier = |d: usize| [vec![0.0; d + 1], vec![1.0; d + 1]].concat();
    let points: Vec<Vec3> = (0..=m).flat_map(|j| (0..=n).map(move |i| net[i][j])).collect();
    let surface = NurbsSurface::new([n, m], [bezier(n), bezier(m)], &points, &vec![1.0; points.len()])
        .map_err(|e| e.to_string())?;
    let top = sheet(&surface)?;
    let (x0, x1, y0, y1) = (net[0][0].x, net[n][0].x, net[0][0].y, net[0][m].y);
    let floor = solidcraft_kernel::patch_region(&Plane::XY, &rect(x0, y0, x1, y1))?;
    let row = |j: usize| (0..=n).map(|i| net[i][j]).collect::<Vec<_>>();
    // the sides' own x runs from the floor's corner at (x0, y0)
    let sides = [
        side(v3(x0, y0, 0.0), Vec3::X, x1 - x0, edge(&row(0), |q| q.x - x0)?)?,
        side(v3(x0, y1, 0.0), Vec3::X, x1 - x0, edge(&row(m), |q| q.x - x0)?)?,
        side(v3(x0, y0, 0.0), Vec3::Y, y1 - y0, edge(&net[0], |q| q.y - y0)?)?,
        side(v3(x1, y0, 0.0), Vec3::Y, y1 - y0, edge(&net[n], |q| q.y - y0)?)?,
    ];
    let faces: Vec<&Body> = [&top, &floor].into_iter().chain(&sides).collect();
    Ok(solidcraft_kernel::stitch(&faces, 1e-6)?)
}

pub fn build() -> Result<Body, Refusal> {
    let (xs, zs) = ([1.0, -1.0 / 3.0, -1.0 / 3.0, 1.0], [-0.5, 1.5, -0.5, 1.5]);
    let net: Vec<Vec<Vec3>> =
        (0..4).map(|i| (0..2).map(|j| v3(xs[i], j as f64, zs[i] + j as f64 / 10.0)).collect()).collect();
    let block = block_under_patch(&piece(&net, 0.55, 0.0)?)?;
    // the plane z = 11y/10: level along x, rising along (0, 1, 11/10); its normal x × y,
    // (0, −11/10, 1), points up and away from the part kept
    let rise = v3(0.0, 1.0, 1.1) * (1.0 / 2.21f64.sqrt());
    let plane = Plane { origin: Vec3::ZERO, x: Vec3::X, y: rise };
    let (below, _) = solidcraft_kernel::half_space(&block, &plane, false)?;
    Ok(below.ok_or("nothing of the block lies below the plane")?)
}
