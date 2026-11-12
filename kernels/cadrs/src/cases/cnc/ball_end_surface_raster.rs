//! A free-form surface finished by a ball-end mill in 41 three-axis raster passes. The plate is
//! the box from (0, 0, −20) to (100, 80, 0); a ball-end mill of radius 3, upright, runs its tip
//! over f(x, y) = −5 + 4 sin(πx/50) sin(πy/40) along the lines y = 0, 2, …, 80, straight from
//! each of 23 points (x = −5, 0, …, 105) to the next, the passes zigzagging.
//!
//! The program is run as written, a pass at a time: what each move sweeps is cadrs's sweep of a
//! solid (one slide aslant the mill's axis: the ball a cylinder between two balls, the shank a
//! slanted prism), and a pass's sweeps are cut from the plate together. The mill is its half
//! section turned.
use crate::bench::{block, p3, revolve, v3};
use crate::shared::cnc::Program;
use crate::shared::geometry::Section;
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel, Plane};
use std::f64::consts::PI;

fn surface(x: f64, y: f64) -> f64 {
    -5.0 + 4.0 * (PI * x / 50.0).sin() * (PI * y / 40.0).sin()
}

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let (min, max) = (p3(0.0, 0.0, -20.0), p3(100.0, 80.0, 0.0));
    let plate = block(k, min, max)?;
    let w = 2f64.sqrt() / 2.0;
    let half = Section::on(Plane::xz(0.0))
        .conic((0.0, 0.0), (3.0, 0.0), w, (3.0, 3.0))
        .line((3.0, 3.0), (3.0, 30.0))
        .line((3.0, 30.0), (0.0, 30.0))
        .line((0.0, 30.0), (0.0, 0.0));
    let mill = revolve(k, &half.sketch(), Axis::new(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0)), None)?;
    let mut p = Program::new(plate, (min, max), mill, 3.0, p3(-5.0, 0.0, 5.0)).gathering();
    for j in 0..=40 {
        let y = 2.0 * j as f64;
        let along: Vec<usize> = if j % 2 == 0 { (0..=22).collect() } else { (0..=22).rev().collect() };
        for i in along {
            let x = -5.0 + 5.0 * i as f64;
            p.line(k, (x, y, surface(x, y)))?;
        }
        p.cut(k)?;
    }
    p.line(k, (-5.0, 80.0, 5.0))?;
    p.cut(k)?;
    Ok(p.body)
}
