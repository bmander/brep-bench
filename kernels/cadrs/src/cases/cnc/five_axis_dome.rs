//! A dome machined by a five-axis program of four cutters. The blank is the cylinder of radius 50
//! about the z axis from z = 0 to 45; the dome aimed at, the ball of radius 40 about the origin.
//! A bull-nose mill (radius 8, corner 2) roughs it upright in eighteen level circles; a ball-end
//! mill of radius 4, held square to the dome, finishes it along 180 meridians (latitude 3° to
//! 75°, 2° apart), round 15 circles near the pole and with a touch at the pole; a flat end mill
//! of radius 5 faces twelve seats at latitude 40°, and a drill of radius 2.5 drills each, both
//! square to the dome.
//!
//! The program is run as written. Each roughing circle's groove is the mill's section through
//! its axis carried upright round the circle (cadrs's Sweep along a path). A leaning cutter is the
//! upright one moved up the z axis, turned down to its latitude and round to its longitude; what
//! each of its moves sweeps is cadrs's sweep of a solid (a slide along its axis, a turn about
//! the level line through the origin square to its meridian, or about the z axis), and a few
//! passes' sweeps are cut from the dome together. The moves between passes, clear of the blank,
//! cut nothing and are left out.
use crate::bench::{arc, boolean, cut, cylinder, moved, p2, p3, revolve, turned, v3, V3};
use crate::shared::cnc::{across, round_the_path};
use crate::shared::geometry::Section;
use crate::shared::swept::{slid, swung, z_axis};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, BoolOp, DefaultKernel, Plane};

const DOME: f64 = 40.0;

/// The dome's outward normal at a latitude and a longitude (degrees).
fn normal(lat: f64, lon: f64) -> V3 {
    let (l, p) = (lat.to_radians(), lon.to_radians());
    v3(l.cos() * p.cos(), l.cos() * p.sin(), l.sin())
}

/// `cutter` (upright, its tip at the origin) square to the dome at a latitude and a longitude,
/// its tip `r` from the origin: moved up the z axis, turned down about the y axis, then round
/// the z axis.
fn square_to(k: &mut DefaultKernel, cutter: BodyId, lat: f64, lon: f64, r: f64) -> Result<BodyId, Refusal> {
    let up = moved(k, cutter, v3(0.0, 0.0, r))?;
    let leaning = turned(k, up, Axis::new(p3(0.0, 0.0, 0.0), v3(0.0, 1.0, 0.0)), 90.0 - lat)?;
    turned(k, leaning, z_axis(), lon)
}

/// A cutter's half section turned about the z axis.
fn turned_section(k: &mut DefaultKernel, half: Section) -> Result<BodyId, Refusal> {
    revolve(k, &half.sketch(), z_axis(), None)
}

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let up = v3(0.0, 0.0, 1.0);
    let blank = cylinder(k, p3(0.0, 0.0, 0.0), up, 50.0, 45.0)?;
    let w = 2f64.sqrt() / 2.0;
    // 1. Roughing: the bull-nose mill, upright, in level circles. Each circle's groove is the
    // mill's section through its axis carried upright round the circle (the plunge and the rise
    // at (R, 0), a point of the circle, are within it).
    let mut dome = blank;
    for z in [41.0f64, 36.0, 31.0, 26.0, 21.0, 16.0, 11.0, 6.0] {
        let rho = (DOME * DOME - z * z).max(0.0).sqrt();
        let mut radii = Vec::new();
        let mut r = rho + 9.0;
        while r < 58.0 {
            radii.push(r);
            r += 12.0;
        }
        for r in radii.into_iter().rev() {
            let section = Section::on(across(r, 0.0, (0.0, 1.0)))
                .line((-6.0, z), (6.0, z))
                .conic((6.0, z), (8.0, z), w, (8.0, z + 2.0))
                .line((8.0, z + 2.0), (8.0, 50.0))
                .line((8.0, 50.0), (-8.0, 50.0))
                .line((-8.0, 50.0), (-8.0, z + 2.0))
                .conic((-8.0, z + 2.0), (-8.0, z), w, (-6.0, z));
            let ring = round_the_path(
                k,
                &section.sketch(),
                vec![arc(p2(0.0, 0.0), r, 0.0, 180.0), arc(p2(0.0, 0.0), r, 180.0, 180.0)],
            )?;
            dome = cut(k, dome, ring)?;
        }
    }
    // What some passes sweep, cut from the dome together.
    let mut swept: Vec<BodyId> = Vec::new();
    // (Each meridian's turn: the cutter where the turn starts and the line it turns about, to
    // make the turn's sweep again in parts.)
    let mut turns: Vec<(BodyId, BodyId, Axis)> = Vec::new();
    // (All at once; or, where that boolean fails, one by one, a sweep whose own cut fails kept
    // for later: what the cutters pass through is the same in any order, and the cut that fails
    // against the dome as it is may not against the dome as the rest leave it.)
    let mut later: Vec<BodyId> = Vec::new();
    let mut together = |k: &mut DefaultKernel, swept: &mut Vec<BodyId>, dome: &mut BodyId, at_least: usize| -> Result<(), Refusal> {
        if swept.len() >= at_least.max(1) {
            match boolean(k, BoolOp::Subtract, *dome, swept) {
                Ok(cut) => *dome = cut,
                Err(_) => {
                    for tool in swept.iter() {
                        match boolean(k, BoolOp::Subtract, *dome, &[*tool]) {
                            Ok(cut) => *dome = cut,
                            Err(_) => later.push(*tool),
                        }
                    }
                }
            }
            swept.clear();
        }
        Ok(())
    };
    // 2. Finishing: the ball-end mill square to the dome.
    let ball = turned_section(
        k,
        Section::on(Plane::xz(0.0))
            .conic((0.0, 0.0), (4.0, 0.0), w, (4.0, 4.0))
            .line((4.0, 4.0), (4.0, 40.0))
            .line((4.0, 40.0), (0.0, 40.0))
            .line((0.0, 40.0), (0.0, 0.0)),
    )?;
    let reach = 90.0 - DOME;
    for pass in 0..180 {
        let lon = 2.0 * pass as f64;
        // In along its axis at latitude 3°, up the meridian to 75°, out along its axis.
        let low = square_to(k, ball, 3.0, lon, DOME)?;
        swept.push(slid(k, low, normal(3.0, lon) * reach)?);
        let level = v3(lon.to_radians().sin(), -lon.to_radians().cos(), 0.0);
        let about = Axis::new(p3(0.0, 0.0, 0.0), level);
        let turn = swung(k, low, about, 0.0, 72.0)?;
        turns.push((turn, low, about));
        swept.push(turn);
        let high = square_to(k, ball, 75.0, lon, DOME)?;
        swept.push(slid(k, high, normal(75.0, lon) * reach)?);
        together(k, &mut swept, &mut dome, 18)?;
    }
    for lat in 75..90 {
        // In along its axis, once round the z axis, out again the way it came.
        let at = square_to(k, ball, lat as f64, 0.0, DOME)?;
        swept.push(slid(k, at, normal(lat as f64, 0.0) * reach)?);
        swept.push(swung(k, at, z_axis(), 0.0, 360.0)?);
        together(k, &mut swept, &mut dome, 18)?;
    }
    let pole = square_to(k, ball, 90.0, 0.0, DOME)?;
    swept.push(slid(k, pole, up * reach)?);
    together(k, &mut swept, &mut dome, 1)?;
    // 3. The seats and 4. the holes, each cutter slid in along the normal and back.
    let flat = cylinder(k, p3(0.0, 0.0, 0.0), up, 5.0, 40.0)?;
    let drill = turned_section(
        k,
        Section::on(Plane::xz(0.0))
            .line((0.0, 0.0), (2.5, 2.5))
            .line((2.5, 2.5), (2.5, 40.0))
            .line((2.5, 40.0), (0.0, 40.0))
            .line((0.0, 40.0), (0.0, 0.0)),
    )?;
    for (cutter, depth) in [(flat, 38.5), (drill, 30.0)] {
        for seat in 0..12 {
            let lon = 15.0 + 30.0 * seat as f64;
            let at = square_to(k, cutter, 40.0, lon, depth)?;
            swept.push(slid(k, at, normal(40.0, lon) * (70.0 - depth))?);
        }
        together(k, &mut swept, &mut dome, 1)?;
    }
    // The sweeps kept for later, while any of them cuts; then a meridian's turn that still won't
    // cut whole is cut in parts (the same sweep, its pieces' ends elsewhere).
    while !later.is_empty() {
        let before = later.len();
        let mut still = Vec::new();
        for tool in later.drain(..) {
            match boolean(k, BoolOp::Subtract, dome, &[tool]) {
                Ok(cut) => dome = cut,
                Err(_) => still.push(tool),
            }
        }
        if still.len() == before {
            break;
        }
        later = still;
    }
    for tool in later {
        let Some(&(_, low, about)) = turns.iter().find(|t| t.0 == tool) else {
            return Err(Refusal::from("cadrs's boolean fails to cut a finishing slide's sweep from the dome"));
        };
        let in_parts = [2, 3, 5, 7].into_iter().find_map(|parts| {
            let mut cut = dome;
            for i in 0..parts {
                let (from, to) = (72.0 * i as f64 / parts as f64, 72.0 * (i + 1) as f64 / parts as f64);
                let piece = swung(k, low, about, from, to).ok()?;
                cut = boolean(k, BoolOp::Subtract, cut, &[piece]).ok()?;
            }
            Some(cut)
        });
        dome = in_parts.ok_or_else(|| {
            Refusal::from("cadrs's boolean fails to cut a finishing pass's sweep from the dome, whole or in parts (it leaves an open shell)")
        })?;
    }
    Ok(dome)
}
