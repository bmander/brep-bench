//! A block on the square 0 ≤ x, y ≤ 20 of the xy plane, floor at z = 0 and four upright sides,
//! under the non-rational Bézier patch of degree 15 each way whose 16 × 16 control points are
//! (20i/15, 20j/15, 4 + ((3i + 5j) mod 7)).
//!
//! next.BREP has no operation that makes a face of a given surface, nor a split by a free-form
//! surface, so the block's six faces are stated on their carriers: the patch on top, bounded by
//! its four edge curves (the sides' top edges, curves of degree 15), and five planes, their
//! pcurves the kernel's.
use crate::solids::geometry::{uv_line, Records};
use crate::solids::p;
use brep_kernel::{make_line, make_plane, BrepSolid, NurbsSurface, Vec3, Vec4};

const DEGREE: usize = 15;

pub fn build() -> Result<BrepSolid, String> {
    let at = |k: usize| 20.0 * k as f64 / DEGREE as f64;
    let height = |i: usize, j: usize| (4 + (3 * i + 5 * j) % 7) as f64;
    let net = (0..=DEGREE)
        .map(|i| {
            (0..=DEGREE).map(|j| Vec4::from_point(p(at(i), at(j), height(i, j)), 1.0)).collect()
        })
        .collect();
    let clamped = [vec![0.0; DEGREE + 1], vec![1.0; DEGREE + 1]].concat();
    let patch = NurbsSurface::new(DEGREE, DEGREE, clamped.clone(), clamped, net)?;

    let mut b = Records::new();
    // corners of the top t[i][j] = S(i, j) and of the floor f[i][j] under them
    let mut t = [[None; 2]; 2];
    let mut f = [[None; 2]; 2];
    let mut tp = [[Vec3::default(); 2]; 2];
    let mut fp = [[Vec3::default(); 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            tp[i][j] = patch.evaluate(i as f64, j as f64)?;
            fp[i][j] = p(tp[i][j].x, tp[i][j].y, 0.0);
            t[i][j] = Some(b.vertex(tp[i][j]));
            f[i][j] = Some(b.vertex(fp[i][j]));
        }
    }
    let (t, f) = (t.map(|r| r.map(Option::unwrap)), f.map(|r| r.map(Option::unwrap)));
    // the patch's edges: along y = 0 and y = 20 (v = 0, 1), along x = 20 and x = 0 (u = 1, 0)
    let top_y0 = b.edge(patch.iso_curve_v(0.0)?, t[0][0], t[1][0])?;
    let top_x1 = b.edge(patch.iso_curve_u(1.0)?, t[1][0], t[1][1])?;
    let top_y1 = b.edge(patch.iso_curve_v(1.0)?, t[0][1], t[1][1])?;
    let top_x0 = b.edge(patch.iso_curve_u(0.0)?, t[0][0], t[0][1])?;
    let floor_y0 = b.edge(make_line(fp[0][0], fp[1][0])?, f[0][0], f[1][0])?;
    let floor_x1 = b.edge(make_line(fp[1][0], fp[1][1])?, f[1][0], f[1][1])?;
    let floor_y1 = b.edge(make_line(fp[0][1], fp[1][1])?, f[0][1], f[1][1])?;
    let floor_x0 = b.edge(make_line(fp[0][0], fp[0][1])?, f[0][0], f[0][1])?;
    let mut up = [[None; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            up[i][j] = Some(b.edge(make_line(fp[i][j], tp[i][j])?, f[i][j], t[i][j])?);
        }
    }
    let up = up.map(|r| r.map(Option::unwrap));

    b.face(patch, vec![
        (top_y0, true, uv_line(0.0, 0.0, 1.0, 0.0)?),
        (top_x1, true, uv_line(1.0, 0.0, 1.0, 1.0)?),
        (top_y1, false, uv_line(1.0, 1.0, 0.0, 1.0)?),
        (top_x0, false, uv_line(0.0, 1.0, 0.0, 0.0)?),
    ]);
    // each plane's parameter directions cross to its outward normal; the patch stays below 10
    let (x, y, z) = (p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), p(0.0, 0.0, 1.0));
    let tall = 12.0;
    b.flat(make_plane(fp[0][0], x, z, 20.0, tall)?,
           &[(floor_y0, true), (up[1][0], true), (top_y0, false), (up[0][0], false)])?;
    b.flat(make_plane(fp[1][0], y, z, 20.0, tall)?,
           &[(floor_x1, true), (up[1][1], true), (top_x1, false), (up[1][0], false)])?;
    b.flat(make_plane(fp[1][1], x.scale(-1.0), z, 20.0, tall)?,
           &[(floor_y1, false), (up[0][1], true), (top_y1, true), (up[1][1], false)])?;
    b.flat(make_plane(fp[0][1], y.scale(-1.0), z, 20.0, tall)?,
           &[(floor_x0, false), (up[0][0], true), (top_x0, true), (up[0][1], false)])?;
    b.flat(make_plane(fp[0][0], y, x, 20.0, 20.0)?,
           &[(floor_x0, true), (floor_y1, true), (floor_x1, false), (floor_y0, false)])?;
    b.solid(0)
}
