//! What the `cnc/` cases share: the cutters, each an upright solid of revolution given by its
//! meridian from its tip, and a program's moves, each swept by the kernel's whole sweep of a solid
//! (`sweep_solid`): a straight move is the cutter slid along it, an arc the cutter turned about
//! the upright line through the arc's centre. The solid is the block less everything swept.
//!
//! A move that keeps the tip at or above the block's top cuts nothing (every cutter reaches only
//! up from its tip), and is passed over.
use super::p;
use super::swept::{less_all, planar, revolved_tool, slide_by, Piece};
use crate::Refusal;
use brep_kernel::{revolve_profile_brep, sweep_solid, BrepSolid, ScrewStep, SolidMotion, Vec3};
use std::f64::consts::{FRAC_PI_2, TAU};

/// A cutter: its meridian from its tip, in (distance from the axis, height above the tip), a
/// closed chain that runs out from the tip, up the cutter and back down its axis.
pub struct Cutter(pub Vec<Piece>);

impl Cutter {
    /// A flat end mill of `radius`, `length` up from its tip, the middle of its flat end.
    pub fn flat(radius: f64, length: f64) -> Cutter {
        Cutter(vec![
            Piece::Line { from: [0.0, 0.0], to: [radius, 0.0] },
            Piece::Line { from: [radius, 0.0], to: [radius, length] },
            Piece::Line { from: [radius, length], to: [0.0, length] },
            Piece::Line { from: [0.0, length], to: [0.0, 0.0] },
        ])
    }

    /// A bull-nose mill of `radius` whose end corner is rounded to `corner`, `length` up.
    pub fn bull(radius: f64, corner: f64, length: f64) -> Cutter {
        let flat = radius - corner;
        Cutter(vec![
            Piece::Line { from: [0.0, 0.0], to: [flat, 0.0] },
            Piece::Arc { centre: [flat, corner], radius: corner, start: -FRAC_PI_2, end: 0.0 },
            Piece::Line { from: [radius, corner], to: [radius, length] },
            Piece::Line { from: [radius, length], to: [0.0, length] },
            Piece::Line { from: [0.0, length], to: [0.0, 0.0] },
        ])
    }

    /// A ball-end mill of `radius`: a half ball whose lowest point is its tip, below a cylinder up
    /// to `length` above the tip.
    pub fn ball(radius: f64, length: f64) -> Cutter {
        Cutter(vec![
            Piece::Arc { centre: [0.0, radius], radius, start: -FRAC_PI_2, end: 0.0 },
            Piece::Line { from: [radius, radius], to: [radius, length] },
            Piece::Line { from: [radius, length], to: [0.0, length] },
            Piece::Line { from: [0.0, length], to: [0.0, 0.0] },
        ])
    }

    /// A drill of `radius` with a point of half-angle `half` (radians), `length` up from its tip.
    pub fn drill(radius: f64, half: f64, length: f64) -> Cutter {
        let shoulder = radius / half.tan();
        Cutter(vec![
            Piece::Line { from: [0.0, 0.0], to: [radius, shoulder] },
            Piece::Line { from: [radius, shoulder], to: [radius, length] },
            Piece::Line { from: [radius, length], to: [0.0, length] },
            Piece::Line { from: [0.0, length], to: [0.0, 0.0] },
        ])
    }
}

/// A program for one upright cutter: where its tip is, and the solids its moves sweep so far.
pub struct Program {
    cutter: Cutter,
    tip: Vec3,
    /// The block's top: a move with its tip at or above it all along cuts nothing.
    top: f64,
    pub swept: Vec<BrepSolid>,
    /// Whether a slide along the axis was built by `lengthened`.
    pub axial: bool,
}

impl Program {
    pub fn new(cutter: Cutter, tip: Vec3, top: f64) -> Program {
        Program { cutter, tip, top, swept: Vec::new(), axial: false }
    }

    fn tool(&self) -> brep_kernel::SolidTool {
        revolved_tool(self.tip, p(0.0, 0.0, 1.0), p(1.0, 0.0, 0.0), &self.cutter.0)
    }

    /// The cutter with its tip at height `low` under the current tip, its top raised by `rise`:
    /// what it sweeps sliding `rise` along its own upright axis, every cutter here widening (or
    /// keeping its width) all the way up, so that each level of it is covered by the level it
    /// reaches at the top of the slide. Made by the kernel's revolve of that meridian, used only
    /// where the kernel's sweep of a solid refuses the slide (`axial`).
    fn lengthened(&self, low: f64, rise: f64) -> Result<BrepSolid, Refusal> {
        let top = self.cutter.0.iter().fold(0.0f64, |m, piece| match *piece {
            Piece::Line { from, to } => m.max(from[1]).max(to[1]),
            Piece::Arc { centre, radius, .. } => m.max(centre[1] + radius),
        });
        let lift = |q: [f64; 2]| if (q[1] - top).abs() <= 1e-12 { [q[0], q[1] + rise] } else { q };
        let pieces: Vec<Piece> = self.cutter.0.iter().map(|piece| match *piece {
            Piece::Line { from, to } => Piece::Line { from: lift(from), to: lift(to) },
            arc => arc,
        }).collect();
        let foot = p(self.tip.x, self.tip.y, low);
        let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
        Ok(revolve_profile_brep(&planar(&pieces, foot, x, z)?, foot, z, TAU)?)
    }

    fn reaches(&self, lowest: f64) -> bool {
        lowest < self.top - 1e-9
    }

    /// A straight move of the tip to `to`, the cutter only sliding.
    pub fn line(&mut self, to: Vec3) -> Result<&mut Program, Refusal> {
        let d = to.sub(self.tip);
        if d.length() <= 1e-12 {
            return Ok(self);
        }
        if self.reaches(self.tip.z.min(to.z)) {
            let along = d.z.abs() >= d.length() * (1.0 - 1e-12);
            match sweep_solid(&self.tool(), &slide_by(d)) {
                Ok(body) => self.swept.push(body),
                Err(refusal) if along => {
                    self.axial = true;
                    let _ = refusal;
                    self.swept.push(self.lengthened(self.tip.z.min(to.z), d.z.abs())?);
                }
                Err(refusal) => return Err(refusal.into()),
            }
        }
        self.tip = to;
        Ok(self)
    }

    /// A move of the tip round the upright line through `centre` (its x and y) to `to`, at one
    /// height, anticlockwise seen from above or clockwise; a whole circle where `to` is the tip.
    pub fn arc(&mut self, centre: [f64; 2], to: Vec3, anticlockwise: bool) -> Result<&mut Program, Refusal> {
        let a0 = (self.tip.y - centre[1]).atan2(self.tip.x - centre[0]);
        let a1 = (to.y - centre[1]).atan2(to.x - centre[0]);
        let mut sweep = if anticlockwise { a1 - a0 } else { a0 - a1 };
        sweep = sweep.rem_euclid(TAU);
        if sweep <= 1e-9 {
            sweep = TAU;
        }
        let turn = if anticlockwise { sweep } else { -sweep };
        if self.reaches(self.tip.z) {
            // A turn of more than half a circle is swept as two halves: the same solid, and the
            // kernel's sweep takes a motion that never brings the cutter back over its start (a
            // whole circle ends where it began).
            let pieces = if sweep > std::f64::consts::PI { 2 } else { 1 };
            for piece in 0..pieces {
                let part = sweep / pieces as f64;
                let (c, s) = ((turn / pieces as f64 * piece as f64).cos(), (turn / pieces as f64 * piece as f64).sin());
                let (x, y) = (self.tip.x - centre[0], self.tip.y - centre[1]);
                let from = p(centre[0] + c * x - s * y, centre[1] + s * x + c * y, self.tip.z);
                let tool = revolved_tool(from, p(0.0, 0.0, 1.0), p(1.0, 0.0, 0.0), &self.cutter.0);
                let motion = SolidMotion {
                    steps: vec![ScrewStep {
                        axis_origin: p(centre[0], centre[1], 0.0),
                        // A clockwise turn is the anticlockwise one about the line pointing down:
                        // the motion's range runs forward either way.
                        axis_direction: p(0.0, 0.0, if anticlockwise { 1.0 } else { -1.0 }),
                        turn: 1.0,
                        advance: 0.0,
                    }],
                    from: 0.0,
                    to: part,
                };
                self.swept.push(sweep_solid(&tool, &motion)?);
            }
        }
        let (c, s) = (turn.cos(), turn.sin());
        let (x, y) = (self.tip.x - centre[0], self.tip.y - centre[1]);
        self.tip = p(centre[0] + c * x - s * y, centre[1] + s * x + c * y, self.tip.z);
        Ok(self)
    }

    /// The tip's position now.
    pub fn tip(&self) -> Vec3 {
        self.tip
    }
}

/// The closed path round the polygon `corners` (anticlockwise, x and y) at height `z`, each
/// corner rounded to `radius` by an arc tangent to both its sides, from the middle of its first
/// side back to it.
pub fn rounded_loop(program: &mut Program, corners: &[[f64; 2]], radius: f64, z: f64) -> Result<(), Refusal> {
    let n = corners.len();
    let unit = |k: usize| {
        let (a, b) = (corners[k % n], corners[(k + 1) % n]);
        let length = (b[0] - a[0]).hypot(b[1] - a[1]);
        [(b[0] - a[0]) / length, (b[1] - a[1]) / length]
    };
    for k in 1..=n {
        let v = corners[k % n];
        let (i, o) = (unit(k - 1), unit(k));
        let left = i[0] * o[1] - i[1] * o[0] > 0.0;
        let side = if left { [-i[1], i[0]] } else { [i[1], -i[0]] };
        let a = p(v[0] - radius * i[0], v[1] - radius * i[1], z);
        let b = p(v[0] + radius * o[0], v[1] + radius * o[1], z);
        program.line(a)?;
        program.arc([a.x + radius * side[0], a.y + radius * side[1]], b, left)?;
    }
    let (f, g) = (corners[0], corners[1 % n]);
    program.line(p((f[0] + g[0]) / 2.0, (f[1] + g[1]) / 2.0, z))?;
    Ok(())
}

/// `block` less everything every program swept.
pub fn cut(block: BrepSolid, programs: Vec<Program>) -> Result<BrepSolid, Refusal> {
    let swept: Vec<_> = programs.into_iter().flat_map(|program| program.swept).collect();
    // The kernel's n-ary boolean takes at most 255 operands: cut in batches.
    let mut result = Ok(block);
    let mut swept = swept;
    while !swept.is_empty() {
        let rest = swept.split_off(swept.len().min(200));
        result = less_all(result?, swept);
        swept = rest;
    }
    result
}

