//! A 2.5-axis program of three tools. The plate is the box from (0, 0, −20) to (160, 110, 0). A
//! flat end mill of radius 5 clears a pocket (the rectangle from (20, 20) to (140, 90), its
//! corners rounded to 12, round an island of radius 15 about (60, 55)) at z = −4, −8 and −12:
//! raster passes along y = 26, 30, …, 82, then a lap round the wall and one round the island. A
//! flat end mill of radius 3 bores the island's top to radius 9, 6 deep, in two circles. A drill
//! of radius 4 with a 90° point pecks four holes 14 deep at the plate's corners.
//!
//! The program is run as written: every move that reaches the plate is cut from it as it is made,
//! cadrs's cut by a moving solid (a slide, or a turn about an upright line); the moves in the air
//! between cuts cut nothing. The mills are circles extruded, the drill its half section turned.
use crate::bench::{block, cylinder, p3, revolve, v3};
use crate::shared::cnc::Program;
use crate::shared::geometry::Section;
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel, Plane};

/// Where the line at `y` crosses the region the pocketing mill's tip keeps to: inside the wall
/// path (the rectangle from (25, 25) to (135, 85), its corners rounded to 7) and outside the
/// island path (the circle of radius 20 about (60, 55)).
fn intervals(y: f64) -> Vec<(f64, f64)> {
    let inset = |d: f64| 32.0 - (49.0 - d * d).sqrt();
    let left = if y < 32.0 {
        inset(32.0 - y)
    } else if y > 78.0 {
        inset(y - 78.0)
    } else {
        25.0
    };
    let right = 160.0 - left;
    let off = (y - 55.0).abs();
    if off < 20.0 {
        let half = (400.0 - off * off).sqrt();
        vec![(left, 60.0 - half), (60.0 + half, right)]
    } else {
        vec![(left, right)]
    }
}

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let (min, max) = (p3(0.0, 0.0, -20.0), p3(160.0, 110.0, 0.0));
    let plate = block(k, min, max)?;
    let up = v3(0.0, 0.0, 1.0);
    let mill = cylinder(k, p3(0.0, 0.0, 0.0), up, 5.0, 30.0)?;
    let mut p = Program::new(plate, (min, max), mill, 5.0, p3(0.0, 0.0, 5.0));
    // The pocket.
    for depth in [-4.0, -8.0, -12.0] {
        for j in 0..15 {
            let y = 26.0 + 4.0 * j as f64;
            for (a, z) in intervals(y) {
                p.line(k, (a, y, 5.0))?;
                p.line(k, (a, y, depth))?;
                p.line(k, (z, y, depth))?;
                p.line(k, (z, y, 5.0))?;
            }
        }
        p.line(k, (80.0, 25.0, 5.0))?;
        p.line(k, (80.0, 25.0, depth))?;
        p.rounded_loop(k, &[(25.0, 25.0), (135.0, 25.0), (135.0, 85.0), (25.0, 85.0)], 7.0)?;
        p.line(k, (80.0, 25.0, 5.0))?;
        p.line(k, (80.0, 55.0, 5.0))?;
        p.line(k, (80.0, 55.0, depth))?;
        p.arc(k, (60.0, 55.0), -360.0)?;
        p.line(k, (80.0, 55.0, 5.0))?;
    }
    // The bore.
    let bore = cylinder(k, p3(0.0, 0.0, 0.0), up, 3.0, 30.0)?;
    p.change(bore, 3.0, p3(60.0, 55.0, 5.0));
    for depth in [-3.0, -6.0] {
        p.line(k, (60.0, 55.0, depth))?;
        p.line(k, (66.0, 55.0, depth))?;
        p.arc(k, (60.0, 55.0), 360.0)?;
        p.line(k, (60.0, 55.0, depth))?;
    }
    p.line(k, (60.0, 55.0, 5.0))?;
    // The holes.
    let half = Section::on(Plane::xz(0.0))
        .line((0.0, 0.0), (4.0, 4.0))
        .line((4.0, 4.0), (4.0, 30.0))
        .line((4.0, 30.0), (0.0, 30.0))
        .line((0.0, 30.0), (0.0, 0.0));
    let drill = revolve(k, &half.sketch(), Axis::new(p3(0.0, 0.0, 0.0), up), None)?;
    p.change(drill, 4.0, p3(60.0, 55.0, 5.0));
    for (x, y) in [(8.0, 8.0), (152.0, 8.0), (152.0, 102.0), (8.0, 102.0)] {
        p.line(k, (x, y, 5.0))?;
        for (down, back) in [(-5.0, 2.0), (-10.0, 2.0), (-14.0, 5.0)] {
            p.line(k, (x, y, down))?;
            p.line(k, (x, y, back))?;
        }
    }
    Ok(p.body)
}
