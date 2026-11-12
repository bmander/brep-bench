//! #3DBenchy, as near as a few solids come to it, from numbers read off the reference.
//!
//! The hull's side is the reference's own surface, its 12 × 6 bicubic net written out (rounded to
//! 0.01), and its mirror image (cadrs's face on a B-spline net): sewn with a ruled stern between
//! their ends and the flat faces their rims bound at z = 0 and z = 30 (cadrs's Fill of edges)
//! into a solid, kept below the sheer (an arc of radius 103.08 aft, the gunwale's plane at the
//! bow). The deck well is a ruled loft through its plan at four heights, read on thirteen rays
//! from the origin: a bilinear face for each side of each band, sewn with the lowest and the
//! highest plans, kept above the deck. The wheelhouse, its room, the roof and the cargo box are
//! each a block cut down to the reference's own planes (the walls leaning 1.5°, 4.5° and 3.2°,
//! the roof 5.5°, the box 5°); the doors, the windows, the chimney, the rod holder, the drain and
//! the hawse pipes are cylinders on the reference's own axes.
use crate::bench::{block, common, cut, cut_each, cylinder, extrude, extrude_symmetric, p3, union_each, v3, P3, V3};
use crate::shared::geometry::Section;
use crate::Refusal;
use cadrs_kernel::{
    BodyId, BsplineSurface, Curve2, DefaultKernel, FillCurve, FillSpec, Kernel, Loop, Plane, Profile,
};
use nalgebra::Point2;

/// The hull's side: its net's rows from the stern to the stem, each the x and y of a pole at each
/// of the heights `Z`.
const Z: [f64; 6] = [0., 3.31, 9.86, 20., 26.67, 30.];
const NET: [[f64; 12]; 12] = [
    [-25., 6.5, -26.58, 8.2, -29.7, 11.55, -30.07, 13.11, -30.02, 13.7, -30., 14.],
    [-16.67, 7., -18.03, 8.92, -20.73, 12.71, -19.33, 13.99, -19.33, 14.22, -19.33, 14.33],
    [-6.95, 7.58, -9.03, 9.65, -13.14, 13.72, -9.06, 14.83, -9.1, 14.71, -9.12, 14.65],
    [1.7, 8.1, 1.04, 10.53, -0.27, 15.33, 4.9, 15.97, 4.78, 15.38, 4.72, 15.09],
    [3.27, 7.91, 3.01, 10.4, 2.47, 15.32, 7.73, 15.81, 7.59, 15.16, 7.52, 14.84],
    [6.12, 7.14, 6.78, 9.59, 8.06, 14.43, 13.32, 14.65, 13.12, 13.96, 13.02, 13.62],
    [7.4, 6.57, 8.58, 8.92, 10.9, 13.55, 16.07, 13.65, 15.85, 12.99, 15.74, 12.65],
    [9.71, 5.17, 11.9, 7.09, 16.24, 10.9, 21.18, 10.88, 20.96, 10.32, 20.85, 10.04],
    [10.74, 4.32, 13.42, 5.94, 18.74, 9.14, 23.54, 9.1, 23.35, 8.63, 23.25, 8.4],
    [12.56, 2.38, 16.08, 3.24, 23.04, 4.96, 27.63, 4.93, 27.57, 4.69, 27.54, 4.56],
    [13.35, 1.27, 17.21, 1.7, 24.84, 2.54, 29.35, 2.55, 29.4, 2.43, 29.43, 2.37],
    [14., 0., 18.1, 0., 26.21, 0., 30.68, 0., 30.89, 0., 31., 0.],
];
const UKNOTS: [f64; 16] =
    [0., 0., 0., 0., 0.5052, 0.5052, 0.6289, 0.6289, 0.7526, 0.7526, 0.8763, 0.8763, 1., 1., 1., 1.];
const VKNOTS: [f64; 10] = [0., 0., 0., 0., 0.3389, 0.6699, 1., 1., 1., 1.];
/// The deck well's plan at each height, its half on y ≥ 0 from the stem round to the stern.
const BOW: [[f64; 2]; 4] = [[27.77, 0.], [26.11, 3.67], [21.05, 7.66], [13.93, 11.69]];
const WELL: [(f64, [[f64; 2]; 9]); 4] = [
    (8.6, [[7.41, 10.59], [3.97, 10.91], [0.97, 11.08], [-1.92, 10.89], [-8.54, 10.18],
           [-17.78, 10.26], [-23.57, 8.58], [-26.17, 4.62], [-27.04, 0.]]),
    (10.5, [[7.41, 10.59], [4.37, 12.02], [1.07, 12.2], [-2.11, 11.97], [-9.38, 11.18],
            [-17.78, 10.26], [-25.8, 9.39], [-27.06, 4.77], [-27.6, 0.]]),
    (12., [[8.22, 11.74], [4.57, 12.56], [1.11, 12.72], [-2.2, 12.45], [-9.38, 11.18],
           [-17.78, 10.26], [-25.8, 9.39], [-27.06, 4.77], [-28.04, 0.]]),
    (28., [[9.54, 13.63], [4.57, 12.56], [1.11, 12.72], [-2.2, 12.45], [-9.38, 11.18],
           [-17.78, 10.26], [-25.8, 9.39], [-27.06, 4.77], [-28.04, 0.]]),
];
/// The deck (x, z), and the well above it.
const DECK: [[f64; 2]; 6] = [[-40., 8.5], [5.95, 8.5], [10.12, 14.46], [40., 23.42], [40., 60.], [-40., 60.]];
/// The aft sheer's circle in (x, z): its centre and its radius; the transom.
const SHEER: [f64; 3] = [-23.0812, 113.7248, 103.0821];
const STERN: f64 = -29.1765;
/// The sines and cosines of the walls' leans: the wheelhouse's back 1.5°, its front 4.5°, the
/// roof 5.5° and the cargo box 5°.
const S15: f64 = 0.02618;
const C15: f64 = 0.99966;
const S45: f64 = 0.07846;
const C45: f64 = 0.99692;
const S55: f64 = 0.09585;
const C55: f64 = 0.99540;
const S5: f64 = 0.08716;
const C5: f64 = 0.99619;

/// The common part of the half-spaces a·x + b·y + c·z ≤ d: a block, what lies beyond each plane
/// cut from it (a slab drawn on the plane and extruded outward).
fn convex(k: &mut DefaultKernel, planes: &[[f64; 4]]) -> Result<BodyId, Refusal> {
    let mut piece = block(k, p3(-60., -60., -60.), p3(60., 60., 60.))?;
    for &[a, b, c, d] in planes {
        let n = v3(a, b, c);
        let on = p3(0., 0., 0.) + n * (d / n.norm_squared());
        let slab = Profile::of(Plane::square_to(on, n), vec![Loop::rectangle(-300., -300., 300., 300.)]);
        let beyond = extrude(k, &slab, 300.)?;
        piece = cut(k, piece, beyond)?;
    }
    Ok(piece)
}

/// The prism through y = ±40 of an outline in the xz plane: corners joined by lines, but where a
/// circle's centre and radius follow a corner, by the short arc of that circle to the next.
fn across(k: &mut DefaultKernel, corners: &[([f64; 2], Option<[f64; 3]>)]) -> Result<BodyId, Refusal> {
    // (Sketch x along x, sketch y up.)
    let mut outline = Section::on(Plane::new(p3(0., 0., 0.), v3(1., 0., 0.), v3(0., -1., 0.)));
    let n = corners.len();
    for i in 0..n {
        let (a, b) = (corners[i].0, corners[(i + 1) % n].0);
        outline = match corners[i].1 {
            None => outline.line((a[0], a[1]), (b[0], b[1])),
            Some([cx, cz, r]) => {
                // The arc as a conic: its end tangents meet on the bisector, weighted by the
                // cosine of half its angle.
                let (ua, ub) = (((a[0] - cx) / r, (a[1] - cz) / r), ((b[0] - cx) / r, (b[1] - cz) / r));
                let len = (ua.0 + ub.0).hypot(ua.1 + ub.1);
                let bis = ((ua.0 + ub.0) / len, (ua.1 + ub.1) / len);
                let w = ua.0 * bis.0 + ua.1 * bis.1;
                outline.conic((a[0], a[1]), (cx + bis.0 * r / w, cz + bis.1 * r / w), w, (b[0], b[1]))
            }
        };
    }
    extrude_symmetric(k, &outline.sketch(), 40.)
}

/// The face on a B-spline net.
fn sheet(
    k: &mut DefaultKernel,
    degree_u: usize,
    knots_u: Vec<f64>,
    degree_v: usize,
    knots_v: Vec<f64>,
    poles: Vec<Vec<P3>>,
) -> Result<BodyId, Refusal> {
    let surface = BsplineSurface { degree_u, degree_v, knots_u, knots_v, poles, weights: Vec::new() };
    Ok(k.bspline_face(&surface, None)?.first()?)
}

/// The side net (its mirror image for `s` = −1) as poles.
fn side(s: f64) -> Vec<Vec<P3>> {
    NET.iter().map(|row| (0..6).map(|j| p3(row[2 * j], s * row[2 * j + 1], Z[j])).collect()).collect()
}

/// The edge of a sheet that lies at the height `z`.
fn rim(k: &DefaultKernel, body: BodyId, z: f64) -> Result<FillCurve, Refusal> {
    let level = |p: &P3| (p.z - z).abs() < 1e-6;
    let edge = k
        .edges(body)?
        .into_iter()
        .find(|e| level(&e.start) && level(&e.mid) && level(&e.end))
        .ok_or("a sheet has no edge at that height")?;
    Ok(FillCurve::Edge { body, edge: edge.id })
}

/// The side net and its mirror image, sewn with the stern and the flat faces at z = 0 and z = 30.
fn hull(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let (port, starboard) = (side(1.), side(-1.));
    let stern = vec![port[0].clone(), starboard[0].clone()];
    let port = sheet(k, 3, UKNOTS.to_vec(), 3, VKNOTS.to_vec(), port)?;
    let starboard = sheet(k, 3, UKNOTS.to_vec(), 3, VKNOTS.to_vec(), starboard)?;
    let stern = sheet(k, 1, vec![0., 0., 1., 1.], 3, VKNOTS.to_vec(), stern)?;
    let mut faces = vec![port, starboard, stern];
    for z in [Z[0], Z[5]] {
        let curves = vec![rim(k, port, z)?, rim(k, stern, z)?, rim(k, starboard, z)?];
        faces.push(k.fill(&FillSpec { curves, source: 1 })?.first()?);
    }
    Ok(k.sew_solid(&faces, 1e-6)?.first()?)
}

/// The deck well's plans lofted: a bilinear face for each side of each band between two plans,
/// sewn with the lowest and the highest.
fn well(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let rings: Vec<Vec<P3>> = WELL
        .iter()
        .map(|(z, aft)| {
            let half: Vec<[f64; 2]> = BOW.iter().chain(aft.iter()).copied().collect();
            // Counter-clockwise about z: up the port side from the stem, back down the starboard.
            half[..12]
                .iter()
                .map(|&[x, y]| p3(x, y, *z))
                .chain(half[1..].iter().rev().map(|&[x, y]| p3(x, -y, *z)))
                .collect()
        })
        .collect();
    let mut faces = Vec::new();
    for ring in [&rings[0], &rings[rings.len() - 1]] {
        let n = ring.len();
        let plane = Plane::xy(ring[0].z);
        let curves = (0..n)
            .map(|i| {
                let (a, b) = (ring[i], ring[(i + 1) % n]);
                FillCurve::Sketch { plane, curve: Curve2::line(Point2::new(a.x, a.y), Point2::new(b.x, b.y)) }
            })
            .collect();
        faces.push(k.fill(&FillSpec { curves, source: 1 })?.first()?);
    }
    let knots = vec![0., 0., 1., 1.];
    for w in rings.windows(2) {
        let n = w[0].len();
        for i in 0..n {
            let poles = vec![vec![w[0][i], w[1][i]], vec![w[0][(i + 1) % n], w[1][(i + 1) % n]]];
            faces.push(sheet(k, 1, knots.clone(), 1, knots.clone(), poles)?);
        }
    }
    Ok(k.sew_solid(&faces, 1e-6)?.first()?)
}

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let [cx, cz, r] = SHEER;
    let sheer = |x: f64| [x, cz - (r * r - (x - cx) * (x - cx)).sqrt()];
    let side = across(
        k,
        &[([STERN, -1.], None), ([32., -1.], None), ([32., 27.89], None), ([10.22, 21.36], None),
          (sheer(4.16), Some(SHEER)), (sheer(STERN), None)],
    )?;
    let deck = across(k, &DECK.iter().map(|&p| (p, None)).collect::<Vec<_>>())?;
    let well = well(k)?;
    let well = common(k, well, deck)?;
    let hull = hull(k)?;
    let boat = common(k, hull, side)?;
    let boat = cut(k, boat, well)?;
    // The wheelhouse: its walls, less its room and the room's upper part over the console.
    let walls = [[C15, 0., S15, -4.7751], [0.05545, -0.99812, 0.02618, -5.1525],
                 [0.05545, 0.99812, 0.02618, -5.1525], [0., 0., -1., -36.614]];
    let room = |k: &mut DefaultKernel, front: f64, floor: f64| {
        let mut planes: Vec<[f64; 4]> = walls.iter().map(|p| p.map(|c| -c)).collect();
        planes.extend([[C45, 0., -S45, front], [0., 0., -1., -floor]]);
        convex(k, &planes)
    };
    let house = convex(
        k,
        &[[-C15, 0., -S15, 6.7751], [C45, 0., -S45, 10.2992], [0., 0., -1., -8.],
          [-0.05545, 0.99812, -0.02618, 7.1525], [-0.05545, -0.99812, -0.02618, 7.1525],
          [0., 0., 1., 36.614]],
    )?;
    let rooms = [room(k, 5.8069, 8.5)?, room(k, 8.2992, 20.5)?];
    let house = cut_each(k, house, &rooms)?;
    let roof = convex(
        k,
        &[[-S55, 0., C55, 38.7973], [C55, 0., S55, 17.7101], [-C55, 0., -S55, 5.2885],
          [-0.05745, 0.99833, -0.00553, 8.8967], [-0.05745, -0.99833, -0.00553, 8.8967],
          [S55, 0., -C55, -35.7973]],
    )?;
    let cargo = convex(
        k,
        &[[-C5, 0., -S5, 16.7295], [0., C5, -S5, 4.7752], [0., -C5, -S5, 4.7752], [0., 0., 1., 15.5],
          [0., 0., -1., -8.], [1., 0., 0., -6.]],
    )?;
    let rod = cylinder(k, p3(-29.939, 0., 22.073), v3(0.28046, 0., -0.95987), 1.75, 20.)?;
    let slab = convex(k, &[[0., 0., 1., 12.87], [0., 0., -1., -8.]])?;
    let rod = common(k, rod, slab)?;
    let up: V3 = v3(0., 0., 1.);
    let stack = [house, roof, cargo, rod, cylinder(k, p3(-4., 0., 36.), up, 3., 9.3)?,
                 cylinder(k, p3(-4., 0., 45.3), up, 3.5, 2.7)?];
    let boat = union_each(k, boat, &stack)?;
    // The cargo box's hold, the chimney's bore, the front and rear windows, the rod holder's bore
    // and the drain.
    let mut cuts = vec![
        block(k, p3(-15.998, -3.998, 6.5), p3(-8.995, 3.998, 16.))?,
        cylinder(k, p3(-4., 0., 37.), up, 1.5, 12.)?,
        block(k, p3(9., -5.25, 24.611), p3(16., 5.25, 34.112))?,
        cylinder(k, p3(-10., 0., 27.1755), v3(C15, 0., S15), 4.5, 6.)?,
        cylinder(k, p3(-24.8065, 0., 4.508), v3(-0.28046, 0., 0.95987), 1., 10.)?,
        cylinder(k, p3(-26.5, 0., 1.6), v3(1., 0., 0.), 1., 6.5)?,
    ];
    for s in [1., -1.] {
        // A door below its arch, the arch, and a hawse pipe.
        cuts.push(convex(
            k,
            &[[-0.99812, -0.0561 * s, -0.02475, 1.3166], [0.99538, 0.0532 * s, -0.07997, 4.7812],
              [0., 0., -1., -8.5], [0., 0., 1., 27.52], [0., -s, 0., -2.], [0., s, 0., 9.5]],
        )?);
        cuts.push(cylinder(k, p3(2.392, 2.0057 * s, 27.5982), v3(-0.05545, s, -0.02618), 4.5, 8.)?);
        cuts.push(cylinder(k, p3(20.6648, 1.0843 * s, 21.4209), v3(0.70711, 0.6984 * s, -0.11062), 2., 12.)?);
    }
    cut_each(k, boat, &cuts)
}
