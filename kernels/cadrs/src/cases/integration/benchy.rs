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
use crate::bench::{block, common, cut, cut_each, cylinder, extrude, extrude_symmetric, p3, union, union_each, v3, P3, V3};
use crate::shared::geometry::Section;
use crate::Refusal;
use cadrs_kernel::{
    BodyId, BsplineSurface, DefaultKernel, FillCurve, FillSpec, Kernel, Loop, OffsetSpec, Plane, Profile,
};

/// The hull's side: its net's rows from the stern to the stem, each the x and y of a pole at each
/// of the heights `Z`.
const Z: [f64; 6] = [0., 3.31262547, 9.86113758, 19.99948838, 26.66940917, 30.];
const NET: [[f64; 12]; 12] = [
    [-25., 6.5, -26.57777641, 8.19743825, -29.69677916, 11.55299264, -30.07483416, 13.10684206, -30.02492292, 13.70254086, -30., 14.],
    [-16.66666667, 7., -18.03251666, 8.91666531, -20.73257597, 12.7055953, -19.33270927, 13.98651841, -19.33312549, 14.21782938, -19.33333333, 14.33333333],
    [-6.95026665, 7.582984, -9.02800892, 9.6451771, -13.13536141, 13.72179144, -9.0559802, 14.83019903, -9.10125023, 14.71160089, -9.12385558, 14.65237951],
    [1.70173289, 8.10210397, 1.0378164, 10.52894472, -0.27463657, 15.32640711, 4.8956376, 15.97006815, 4.78090388, 15.37984019, 4.72361218, 15.08511288],
    [3.27390994, 7.90566507, 3.00533353, 10.39680401, 2.47440245, 15.32137327, 7.73446515, 15.80894762, 7.58959184, 15.1603348, 7.51725009, 14.8364533],
    [6.12303775, 7.14157538, 6.77506979, 9.59134478, 8.06402917, 14.43413329, 13.32229534, 14.65025381, 13.12364577, 13.96396373, 13.02445113, 13.62126829],
    [7.40374132, 6.57336961, 8.57825828, 8.91726822, 10.90008385, 13.55076773, 16.06944082, 13.6522451, 15.84751934, 12.98681943, 15.73670399, 12.65454253],
    [9.71034637, 5.16566225, 11.90317743, 7.0926504, 16.23804139, 10.90198694, 21.17885739, 10.87503743, 20.95982379, 10.32029188, 20.85045048, 10.04328252],
    [10.73847272, 4.32482338, 13.42488446, 5.94169581, 18.7354757, 9.13798492, 23.53919073, 9.09648792, 23.34674437, 8.6314171, 23.25064727, 8.39918636],
    [12.55886666, 2.37589024, 16.07946535, 3.24321782, 23.03910609, 4.95778086, 27.62607126, 4.93455077, 27.56963652, 4.68590901, 27.54145612, 4.56175102],
    [13.3525947, 1.26641564, 17.21234735, 1.69588126, 24.84243935, 2.5448637, 29.35130573, 2.55271285, 29.4045503, 2.43042649, 29.4311377, 2.36936342],
    [14., 0., 18.10315969, 0., 26.21442708, 0., 30.6839943, 0., 30.89475682, 0., 31., 0.],
];
const UKNOTS: [f64; 16] = [
    0., 0., 0., 0., 0.505209458052, 0.505209458052, 0.628907093539, 0.628907093539, 0.752604729026,
    0.752604729026, 0.876302364513, 0.876302364513, 1., 1., 1., 1.,
];
const VKNOTS: [f64; 10] = [0., 0., 0., 0., 0.33885526461, 0.669860758304, 1., 1., 1., 1.];
/// The hull's wall: its inner side is its outer side 2 further in.
const WALL: f64 = 2.;
/// The deck (x, z), and the well above it.
const DECK: [[f64; 2]; 6] =
    [[-40., 8.5], [6., 8.5], [10.23908933, 14.5], [40., 23.42349817], [40., 60.], [-40., 60.]];
/// The aft sheer's circle in (x, z): its centre and its radius; the transom.
const SHEER: [f64; 3] = [-23.0812, 113.7248, 103.0821];
const STERN: f64 = -29.1765;
/// How deep the rail is under the sheer, its upright outer side's circle in (x, y) (mirrored for
/// the other side) and where it ends forward.
const RAIL: f64 = 1.7;
const UPRIGHT: [f64; 3] = [-57.949424, 330.686828, 320.92693];
const AFT_RAIL: f64 = -1.009;
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

/// A rectangle [u0, u1] × [v0, v1] with its corners rounded 0.5, on the plane through `origin`
/// with sketch x along `x` and normal `normal`, extruded `depth` along the normal.
fn rounded(k: &mut DefaultKernel, origin: P3, x: V3, normal: V3, [u0, u1, v0, v1]: [f64; 4], depth: f64)
    -> Result<BodyId, Refusal> {
    use std::f64::consts::FRAC_PI_2 as Q;
    let r = 0.5;
    let outline = Loop::from_point([u0 + r, v0])
        .line_to([u1 - r, v0])
        .arc_about([u1 - r, v0 + r], Q)
        .line_to([u1, v1 - r])
        .arc_about([u1 - r, v1 - r], Q)
        .line_to([u0 + r, v1])
        .arc_about([u0 + r, v1 - r], Q)
        .line_to([u0, v0 + r])
        .arc_about([u0 + r, v0 + r], Q)
        .close();
    extrude(k, &Profile::of(Plane::new(origin, x, normal), vec![outline]), depth)
}

/// The hull with its sides and its stern moved in by the wall: what the well is cut from.
fn inside(k: &mut DefaultKernel, hull: BodyId) -> Result<BodyId, Refusal> {
    let curved = k.faces(hull)?.iter().filter(|f| f.plane.is_none()).map(|f| (f.id, -WALL)).collect();
    Ok(k.offset(hull, &OffsetSpec { distance: 0., faces: curved, sharp: true })?.first()?)
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
    let hull = hull(k)?;
    let well = inside(k, hull)?;
    let well = common(k, well, deck)?;
    let boat = common(k, hull, side)?;
    let boat = cut(k, boat, well)?;
    // The rail aft of the wheelhouse: 1.7 deep under the sheer, its outer side upright (a
    // cylinder about z on each side) where the hull flares out below.
    let rail = cylinder(k, p3(cx, -20., cz), v3(0., 1., 0.), r + RAIL, 40.)?;
    let mut slivers = Vec::new();
    for s in [1., -1.] {
        let wall = cylinder(k, p3(UPRIGHT[0], s * UPRIGHT[1], -10.), v3(0., 0., 1.), UPRIGHT[2], 60.)?;
        let aft = block(k, p3(-40., s.min(0.) * 20., -10.), p3(AFT_RAIL, s.max(0.) * 20., 50.))?;
        let aft = common(k, aft, rail)?;
        slivers.push(common(k, aft, wall)?);
    }
    let boat = cut_each(k, boat, &slivers)?;
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
          [0., 0., 1., 38.]],
    )?;
    let rooms = [room(k, 5.8069, 8.5)?, room(k, 8.2992, 20.5)?];
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
    let mut stack = stack.to_vec();
    for s in [1., -1.] {
        // A door's frame: 1.5 round the doorway, 0.3 proud of the wall.
        let slab = [[-0.05545, 0.99812 * s, -0.02618, 7.4525], [0.05545, -0.99812 * s, 0.02618, -6.9]];
        let mut jambs = vec![[-0.99812, -0.0561 * s, -0.02475, 2.8166], [0.99538, 0.0532 * s, -0.07997, 6.2812],
                             [0., 0., -1., -8.5], [0., 0., 1., 27.52]];
        jambs.extend(slab);
        let jambs = convex(k, &jambs)?;
        let arch = cylinder(k, p3(2.392 - 0.05545 * s * 4., 2.0057 * s + 4. * s, 27.5982 - 0.02618 * 4.),
                            v3(-0.05545, s, -0.02618), 6., 8.)?;
        let above = convex(k, &[slab[0], slab[1], [0., 0., -1., -27.52]])?;
        let arch = common(k, arch, above)?;
        stack.push(union(k, jambs, arch)?);
    }
    // The front window's frame: 1.5 round the window, 0.3 proud of the wall, square to it.
    let front = v3(C45, 0., -S45);
    stack.push(rounded(k, p3(0., 0., 0.) + front * 10., v3(0., 1., 0.), front, [-6.75, 6.75, 24.0213, 36.5512], 0.5992)?);
    let boat = union_each(k, boat, &stack)?;
    // (The rooms after the roof: the house's ceiling is flat under its sloping roof.)
    let boat = cut_each(k, boat, &rooms)?;
    // The wheel on the console, square to the front wall: a disk 1.5 thick, its face sunk 0.5
    // between its rim and its hub.
    let ahead = v3(C45, 0., -S45);
    let face = p3(1.581, 0., 20.086) + ahead * 5.0569;
    let disk = cylinder(k, face, ahead, 5., 1.5)?;
    let sunk = cylinder(k, face - ahead, ahead, 3.5, 1.5)?;
    let hub = cylinder(k, face, ahead, 1.25, 0.5)?;
    let wheel = cut(k, disk, sunk)?;
    let boat = union_each(k, boat, &[wheel, hub])?;
    // The cargo box's hold, the chimney's bore, the front and rear windows, the rod holder's bore
    // and the drain.
    let mut cuts = vec![
        block(k, p3(-15.998, -3.998, 6.5), p3(-8.995, 3.998, 16.))?,
        cylinder(k, p3(-4., 0., 37.), up, 1.5, 12.)?,
        rounded(k, p3(9., 0., 0.), v3(0., 1., 0.), v3(1., 0., 0.), [-5.25, 5.25, 24.611, 34.112], 7.)?,
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
