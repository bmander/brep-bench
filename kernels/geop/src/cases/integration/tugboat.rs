//! A tugboat test print (a benchy): 60 long, 30 wide and 48 high, symmetric about y = 0, its stern
//! at x = 0, its bow toward +x and the flat of its keel on z = 0. The union of six parts less the
//! openings cut into them:
//!
//! 1. the hull, the common part of the plan P extruded along z, the profile S extruded along y and
//!    the chined section z ≥ |y| − 10 extruded along x. P: the transom x = 0 (|y| ≤ 10), quarter
//!    circles of radius 5 about (5, ±10), the sides y = ±15 to x = 30, and arcs of radius 37.5
//!    about (30, ∓22.5) meeting at the stem (60, 0). S: below the sheer z = 14 + x/10 and above
//!    the keel line, the counter (radius 20 about (x, z) = (12, 20), from (0, 4) to (12, 0)), the
//!    flat keel z = 0 to x = 40, and the stem (radius 25 about (40, 25), from (40, 0) to (60, 10));
//! 2. the deck well taken out of it: z ≥ 9 over the plan moved in by 1.5;
//! 3. the wheelhouse: walls between the rounded rectangles [12, 32] × [−10, 10] (radius 2) and
//!    [13.5, 30.5] × [−8.5, 8.5] (radius 0.5), from the deck up to the roof's underside
//!    z = 30 + (x − 12)/10; the roof, a slab 1.5 thick above that plane over [10, 34] × [−12, 12]
//!    (radius 3); arched doors (16 ≤ x ≤ 22, 9 ≤ z ≤ 20 and a half disc of radius 3 about
//!    (19, 20)) through both side walls; a front window (|y| ≤ 7, 22 ≤ z ≤ 28, radius 1.5) and a
//!    round rear window (radius 4 about (y, z) = (0, 22));
//! 4. the chimney, radius 4 about the vertical through (18, 0), from z = 31.5 (inside the roof)
//!    to 48, bored to radius 2.75 down to z = 38;
//! 5. the cargo box, [2.5, 9.5] × [−5, 5] × [9, 14] less [3.5, 8.5] × [−4, 4] × [10, 14];
//! 6. the rod holder, a tube of radius 1.5 about the line through (8, 9, 9) along
//!    (−1/2, 0, √3/2), between the planes z = 8 and z = 15, bored to radius 0.9 down to z = 10;
//! 7. and the hawse holes, radius 1.25 about the line along y through (52, 0, 16).
//!
//! The part's program as a designer writes it: each region of a plane sketched and extruded past
//! where it is cut — the hull as one extrude kept in common with two more, the sloping walls' tops
//! and roof as the slab between two sloping lines sketched from the side — joined, cut and
//! intersected step by step.
use crate::shared::blends::rounded_rectangle;
use crate::shared::booleans::{boolean, extrude_between};
use crate::shared::paths::{drawn_at, offset_plane, plane_square_to};
use crate::shared::{Drawing, extrude, extrude_symmetric, made, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::sketch::SketchPlane;
use geop_ops_model::{BooleanOp, Combine};
use std::f64::consts::PI;

/// A side of a closed outline, from its corner to the next: a line, or the arc about a centre the
/// short way round.
enum Side { Line, Arc([f64; 2]) }
use Side::{Arc, Line};

/// Draws the closed outline whose corners are `corners`, each with the side from it to the next,
/// at coordinates `at` takes a point of the plane to (a sketch on the xz plane is drawn in
/// (x, −z)); an arc turns the short way from its corner to the next, measured as drawn.
fn outline(d: &mut Drawing, corners: &[([f64; 2], Side)], at: fn([f64; 2]) -> [f64; 2]) {
    let points: Vec<_> = corners.iter().map(|(p, _)| d.point(at(*p))).collect();
    for (i, (p, side)) in corners.iter().enumerate() {
        let (j, q) = ((i + 1) % corners.len(), corners[(i + 1) % corners.len()].0);
        match *side {
            Line => { d.line(points[i], points[j]); }
            Arc(c) => {
                let (c, a, b) = (at(c), at(*p), at(q));
                let turn = (b[1] - c[1]).atan2(b[0] - c[0]) - (a[1] - c[1]).atan2(a[0] - c[0]);
                let turn = (turn + PI).rem_euclid(2.0 * PI) - PI;
                d.arc(points[i], points[j], turn.to_degrees());
            }
        }
    }
}

fn as_drawn(p: [f64; 2]) -> [f64; 2] { p }
fn on_zx(p: [f64; 2]) -> [f64; 2] { zx_at(p[0], p[1]) }

/// The hull's plan moved in by `inset`, in (x, y): the transom, the stern's quarter circles, the
/// sides and the bow's two arcs, which meet on y = 0.
fn plan(inset: f64) -> Drawing {
    let (side, bow) = (15.0 - inset, 37.5 - inset);
    let tip = 30.0 + (bow * bow - 22.5 * 22.5).sqrt();
    let mut d = Drawing::new();
    outline(&mut d, &[
        ([inset, -10.0], Line), ([inset, 10.0], Arc([5.0, 10.0])), ([5.0, side], Line),
        ([30.0, side], Arc([30.0, -22.5])), ([tip, 0.0], Arc([30.0, 22.5])),
        ([30.0, -side], Line), ([5.0, -side], Arc([5.0, -10.0])),
    ], as_drawn);
    d
}

/// The plane at `distance` along the origin's axis `normal`, drawn as the origin's plane is.
fn level(b: &mut Bench, id: &str, normal: FrameAxis, distance: f64) -> Result<SketchPlane, Refusal> {
    offset_plane(b, id, normal, distance)
}

/// The slab between the sloping planes z = z0 + (x − 12)/10 and z = z1 + (x − 12)/10, sketched
/// from the side and extruded 40 across, as the step `id`, kept in common with `with`: the
/// solid it leaves.
fn slab(b: &mut Bench, id: &str, z0: f64, z1: f64, with: &str) -> Result<String, Refusal> {
    let at = |x: f64, z: f64| ([x, z + (x - 12.0) / 10.0], Line);
    let mut d = Drawing::new();
    outline(&mut d, &[at(-5.0, z0), at(65.0, z0), at(65.0, z1), at(-5.0, z1)], on_zx);
    let section = format!("{id}_side");
    sketch(b, &section, zx(), d)?;
    extrude_symmetric(b, id, &section, 40.0, Combine::intersect(with.to_string()))?;
    Ok(made("extrude", id))
}

fn hull(b: &mut Bench) -> Result<String, Refusal> {
    let below = level(b, "below_keel", FrameAxis::Z, -2.0)?;
    sketch(b, "plan", below, plan(0.0))?;
    extrude(b, "hull", "plan", 30.0, Combine::NewBody)?;
    // the profile, run past x = 0 and x = 60 along its circles: the counter from x = −1, the stem
    // to x = 62, then up past the sheer and back along it
    let counter = [-1.0, 20.0 - (400.0f64 - 13.0 * 13.0).sqrt()];
    let stem = [62.0, 25.0 - (625.0f64 - 22.0 * 22.0).sqrt()];
    let mut profile = Drawing::new();
    outline(&mut profile, &[
        (counter, Arc([12.0, 20.0])), ([12.0, 0.0], Line), ([40.0, 0.0], Arc([40.0, 25.0])),
        (stem, Line), ([62.0, 20.2], Line), ([-1.0, 13.9], Line),
    ], on_zx);
    sketch(b, "profile", zx(), profile)?;
    extrude_symmetric(b, "sides", "profile", 40.0, Combine::intersect(made("extrude", "hull")))?;
    // the chines, z ≥ |y| − 10: a V whose point is under the keel, closed high above the sheer
    let aft = level(b, "aft", FrameAxis::X, -2.0)?;
    let mut section = Drawing::new();
    outline(&mut section, &[
        ([0.0, -10.0], Line), ([30.0, 20.0], Line), ([30.0, 30.0], Line), ([-30.0, 30.0], Line),
        ([-30.0, 20.0], Line),
    ], as_drawn);
    sketch(b, "section", aft, section)?;
    extrude(b, "chines", "section", 66.0, Combine::intersect(made("extrude", "sides")))?;
    let deck = level(b, "deck", FrameAxis::Z, 9.0)?;
    sketch(b, "well_plan", deck, plan(1.5))?;
    extrude(b, "well", "well_plan", 20.0, Combine::cut(made("extrude", "chines")))?;
    Ok(made("extrude", "well"))
}

fn wheelhouse(b: &mut Bench) -> Result<String, Refusal> {
    let deck = SketchPlane::Datum(geop_core_program::element::ElementReference::new(
        geop_ops_model::datums::datums(), "deck"));
    let mut outer = Drawing::new();
    rounded_rectangle(&mut outer, [12.0, -10.0], [32.0, 10.0], 2.0);
    sketch(b, "walls_outer", deck, outer)?;
    extrude(b, "walls", "walls_outer", 30.0, Combine::NewBody)?;
    let under_deck = level(b, "under_deck", FrameAxis::Z, 8.0)?;
    let mut inner = Drawing::new();
    rounded_rectangle(&mut inner, [13.5, -8.5], [30.5, 8.5], 0.5);
    sketch(b, "walls_inner", under_deck, inner)?;
    extrude(b, "room", "walls_inner", 31.0, Combine::cut(made("extrude", "walls")))?;
    let walls = slab(b, "walls_tops", -30.0, 30.0, &made("extrude", "room"))?;
    let above = level(b, "above", FrameAxis::Z, 25.0)?;
    let mut roof_plan = Drawing::new();
    rounded_rectangle(&mut roof_plan, [10.0, -12.0], [34.0, 12.0], 3.0);
    sketch(b, "roof_plan", above, roof_plan)?;
    extrude(b, "roof", "roof_plan", 15.0, Combine::NewBody)?;
    let roof = slab(b, "roof_slab", 30.0, 31.5, &made("extrude", "roof"))?;
    boolean(b, "house", &walls, &roof, BooleanOp::Union)?;

    let port = level(b, "port", FrameAxis::Y, -11.0)?;
    let mut door = Drawing::new();
    outline(&mut door, &[
        ([16.0, 9.0], Line), ([22.0, 9.0], Line), ([22.0, 20.0], Arc([19.0, 20.0])),
        ([19.0, 23.0], Arc([19.0, 20.0])), ([16.0, 20.0], Line),
    ], on_zx);
    sketch(b, "door", port, door)?;
    extrude(b, "doors", "door", 22.0, Combine::cut(made("boolean", "house")))?;
    let front = level(b, "front", FrameAxis::X, 30.0)?;
    let mut window = Drawing::new();
    rounded_rectangle(&mut window, [-7.0, 22.0], [7.0, 28.0], 1.5);
    sketch(b, "window", front, window)?;
    extrude(b, "front_window", "window", 3.0, Combine::cut(made("extrude", "doors")))?;
    let back = level(b, "back", FrameAxis::X, 11.0)?;
    let mut rear = Drawing::new();
    rear.circle([0.0, 22.0], 4.0);
    sketch(b, "rear", back, rear)?;
    extrude(b, "rear_window", "rear", 3.0, Combine::cut(made("extrude", "front_window")))?;
    Ok(made("extrude", "rear_window"))
}

fn rod_holder(b: &mut Bench) -> Result<(String, String), Refusal> {
    let s3 = 3.0f64.sqrt();
    let lean = [-0.5, 0.0, s3 / 2.0];
    // the axis from where it is at z = 6, well under the deck, for 12 along it (to z ≈ 16.4)
    let start = [8.0 + s3, 9.0, 6.0];
    let across = plane_square_to(b, "across", start, lean)?;
    let centre = drawn_at(start, lean, start);
    let mut tube = Drawing::new();
    tube.circle(centre, 1.5);
    sketch(b, "tube", across.clone(), tube)?;
    extrude(b, "rod", "tube", 12.0, Combine::NewBody)?;
    let mut bore = Drawing::new();
    bore.circle(centre, 0.9);
    sketch(b, "rod_bore", across, bore)?;
    extrude(b, "bore", "rod_bore", 12.0, Combine::NewBody)?;
    // cut off by the levels z = 8 and z = 15, the bore by z = 10
    let mut beside = Drawing::new();
    beside.rectangle([0.0, 5.0], [14.0, 13.0]);
    let low = level(b, "rod_foot", FrameAxis::Z, 8.0)?;
    sketch(b, "rod_levels", low, beside)?;
    extrude(b, "rod_ends", "rod_levels", 7.0, Combine::intersect(made("extrude", "rod")))?;
    let mut beside = Drawing::new();
    beside.rectangle([0.0, 5.0], [14.0, 13.0]);
    let floor = level(b, "bore_floor", FrameAxis::Z, 10.0)?;
    sketch(b, "bore_levels", floor, beside)?;
    extrude(b, "bore_end", "bore_levels", 10.0, Combine::intersect(made("extrude", "bore")))?;
    Ok((made("extrude", "rod_ends"), made("extrude", "bore_end")))
}

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let hull = hull(b)?;
    let house = wheelhouse(b)?;
    boolean(b, "with_house", &hull, &house, BooleanOp::Union)?;
    let boat = made("boolean", "with_house");
    // the chimney and its bore
    let stack_foot = level(b, "stack_foot", FrameAxis::Z, 31.5)?;
    let mut stack = Drawing::new();
    stack.circle([18.0, 0.0], 4.0);
    sketch(b, "stack", stack_foot, stack)?;
    extrude(b, "chimney", "stack", 16.5, Combine::join(boat))?;
    let flue_foot = level(b, "flue_foot", FrameAxis::Z, 38.0)?;
    let mut flue = Drawing::new();
    flue.circle([18.0, 0.0], 2.75);
    sketch(b, "flue", flue_foot, flue)?;
    extrude(b, "chimney_bore", "flue", 11.0, Combine::cut(made("extrude", "chimney")))?;
    // the cargo box and its hold
    let deck = SketchPlane::Datum(geop_core_program::element::ElementReference::new(
        geop_ops_model::datums::datums(), "deck"));
    let mut cargo = Drawing::new();
    cargo.rectangle([2.5, -5.0], [9.5, 5.0]);
    sketch(b, "cargo", deck, cargo)?;
    extrude(b, "cargo_box", "cargo", 5.0, Combine::join(made("extrude", "chimney_bore")))?;
    let hold_floor = level(b, "hold_floor", FrameAxis::Z, 10.0)?;
    let mut hold = Drawing::new();
    hold.rectangle([3.5, -4.0], [8.5, 4.0]);
    sketch(b, "hold", hold_floor, hold)?;
    extrude(b, "cargo_hold", "hold", 5.0, Combine::cut(made("extrude", "cargo_box")))?;
    // the rod holder, joined and then bored
    let (rod, bore) = rod_holder(b)?;
    boolean(b, "with_rod", &made("extrude", "cargo_hold"), &rod, BooleanOp::Union)?;
    boolean(b, "bored", &made("boolean", "with_rod"), &bore, BooleanOp::Difference)?;
    let boat = made("boolean", "bored");
    // the hawse holes, through both bulwarks of the bow
    let starboard = level(b, "starboard", FrameAxis::Y, -12.0)?;
    let mut hawse = Drawing::new();
    hawse.circle(zx_at(52.0, 16.0), 1.25);
    sketch(b, "hawse", starboard, hawse)?;
    extrude_between(b, "hawse_holes", "hawse", 0.0, 24.0, Combine::cut(boat))
}
