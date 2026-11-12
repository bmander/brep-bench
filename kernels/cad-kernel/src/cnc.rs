//! What the `cnc/` cases share: the cutters, drawn upright with their tip at a point, and a
//! program's moves made into the kernel's path of pieces, each move a constant twist (a slide, a
//! turn, a screw) taken up where the move before it left the cutter.
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::{Frame, V};
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Piece, Pose, Screw, Trajectory};
use cad_kernel::util::space::{add, cross, norm, scale, sub};
use std::f64::consts::{FRAC_PI_2, TAU};

/// A cutter, a solid of revolution about its axis, by the shape of its end.
#[derive(Clone, Copy, Debug)]
pub enum Cutter {
    /// A flat end mill: a cylinder of `radius`, `length` long from its tip up.
    Flat { radius: f64, length: f64 },
    /// A bull-nose mill: a flat end mill whose end corner is rounded to `corner`.
    Bull { radius: f64, corner: f64, length: f64 },
    /// A ball-end mill: a half ball of `radius` at its end, its tip the ball's lowest point.
    Ball { radius: f64, length: f64 },
    /// A drill: a cylinder of `radius` ending in a cone of half-angle `half` (radians) to a point.
    Drill { radius: f64, half: f64, length: f64 },
}

/// `cutter` drawn with its tip at `tip` and its axis up: its meridian in the half-plane y = 0
/// beyond the axis, out from the tip along its end to the foot of its side, up its side, back
/// across its top and down its axis, turned once about the axis.
pub fn cutter(c: Cutter, tip: V) -> Result<Brep, String> {
    let at = |x: f64, z: f64| [tip[0] + x, tip[1], tip[2] + z];
    let line = |a: V, b: V| ProfileEdge::Line { a, b };
    // the end, from the tip to the foot of the side; the radius and the length
    let (mut edges, foot, radius, length) = match c {
        Cutter::Flat { radius, length } => (vec![line(at(0., 0.), at(radius, 0.))], 0., radius, length),
        Cutter::Bull { radius, corner, length } => {
            let frame = Frame::new(at(radius - corner, corner), [0., -1., 0.], [1., 0., 0.]);
            (vec![line(at(0., 0.), at(radius - corner, 0.)),
                  ProfileEdge::Arc { frame, r: corner, span: Some([-FRAC_PI_2, 0.]) }],
             corner, radius, length)
        }
        Cutter::Ball { radius, length } => {
            let frame = Frame::new(at(0., radius), [0., -1., 0.], [1., 0., 0.]);
            (vec![ProfileEdge::Arc { frame, r: radius, span: Some([-FRAC_PI_2, 0.]) }], radius, radius, length)
        }
        Cutter::Drill { radius, half, length } => {
            let shoulder = radius / half.tan();
            (vec![line(at(0., 0.), at(radius, shoulder))], shoulder, radius, length)
        }
    };
    edges.push(line(at(radius, foot), at(radius, length)));
    edges.push(line(at(radius, length), at(0., length)));
    edges.push(line(at(0., length), at(0., 0.)));
    let meridian = Profile { origin: tip, normal: [0., 1., 0.], loops: vec![edges], names: Vec::new() };
    revolve(&meridian, tip, [0., 0., 1.], TAU)
}

/// A program for one cutter: where it is drawn, and each move from there, made into the kernel's
/// path of pieces over a roll that is, move by move, the tip's distance travelled (a turn's arc,
/// or its angle where the tip is on the turn's line).
pub struct Program {
    pieces: Vec<Piece>,
    /// The cutter's tip and axis as drawn.
    drawn: V,
    /// Where the moves so far left the cutter, as a pose of it as drawn, and the roll there.
    pose: Pose,
    roll: f64,
}

/// The pose that turns the upright z axis onto the unit `axis` by the least turn (about their
/// common square), and then carries the origin to `tip`: a cutter drawn upright with its tip at the
/// origin, stood at `tip` along `axis`.
pub fn standing(tip: V, axis: V) -> Pose {
    let k = cross([0., 0., 1.], axis);
    let (s, c) = (norm(k), axis[2]);
    let r = if s < 1e-15 {
        let d = if c > 0. { 1. } else { -1. };
        [[1., 0., 0.], [0., d, 0.], [0., 0., d]]
    } else {
        let k = scale(k, 1. / s);
        let m = [[0., -k[2], k[1]], [k[2], 0., -k[0]], [-k[1], k[0], 0.]];
        std::array::from_fn(|i| std::array::from_fn(|j| {
            let mm: f64 = (0..3).map(|l| m[i][l] * m[l][j]).sum();
            (if i == j { 1. } else { 0. }) + s * m[i][j] + (1. - c) * mm
        }))
    };
    Pose { r, p: tip }
}

impl Program {
    /// A program whose cutter is drawn standing upright at `tip`, and starts there.
    pub fn from(tip: V) -> Program {
        let pose = Pose { r: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], p: [0.; 3] };
        Program { pieces: Vec::new(), drawn: tip, pose, roll: 0. }
    }

    /// A program whose cutter is drawn upright with its tip at the origin, and starts at `pose`.
    pub fn placed(pose: Pose) -> Program {
        Program { pieces: Vec::new(), drawn: [0.; 3], pose, roll: 0. }
    }

    /// Where the cutter is drawn: its tip, its axis upright.
    pub fn drawn(&self) -> V { self.drawn }

    /// Where the tip is, and the cutter's axis.
    pub fn tip(&self) -> V { self.pose.point(self.drawn) }
    pub fn axis(&self) -> V { self.pose.vector([0., 0., 1.]) }

    /// The constant twist `twist` taken up for `length` of roll from where the cutter stands.
    fn take(&mut self, twist: Screw, length: f64) {
        let start = self.roll;
        let path = Trajectory::Placed { twist, at: self.pose.then(twist.pose(-start)) };
        self.roll += length;
        self.pose = self.pose.then(twist.pose(length));
        self.pieces.push(Piece { over: [start, self.roll], path });
    }

    /// A straight move of the tip to `to`, the cutter not turning (G0, G1). A move to where the
    /// tip already is, is no move.
    pub fn line(&mut self, to: V) -> &mut Self {
        let d = sub(to, self.tip());
        let length = norm(d);
        if length > 0. {
            self.take(Screw { origin: [0.; 3], axis: scale(d, 1. / length), ratio: 0., advance: TAU }, length);
        }
        self
    }

    /// A slide of the cutter along its own axis, by `by` (outward for more than nought).
    pub fn along_axis(&mut self, by: f64) -> &mut Self {
        let to = add(self.tip(), scale(self.axis(), by));
        self.line(to)
    }

    /// A move of the tip round the circle about the upright line through `centre` to `to`,
    /// anticlockwise seen from above (G3) or clockwise (G2), rising evenly from its height to
    /// `to`'s (a helix where they differ), less than a whole turn unless `turns` adds whole turns
    /// (a whole circle where `to` is the tip). The cutter turns with it about that line, which
    /// leaves a solid of revolution about its own upright axis where it would stand had it only
    /// been carried round.
    pub fn arc(&mut self, centre: [f64; 2], to: V, anticlockwise: bool, turns: u32) -> &mut Self {
        let c = [centre[0], centre[1], 0.];
        let (a, b) = (sub(self.tip(), c), sub(to, c));
        let r = a[0].hypot(a[1]);
        let mut sweep = (a[0] * b[1] - a[1] * b[0]).atan2(a[0] * b[0] + a[1] * b[1]);
        if !anticlockwise { sweep = -sweep }
        if sweep <= 1e-12 { sweep += TAU }
        sweep += TAU * turns as f64;
        // (a rise of no more than a rounding's is none: a level arc is a turn, not a screw)
        let rise = if (b[2] - a[2]).abs() <= 1e-9 * (1. + r) { 0. } else { b[2] - a[2] };
        let length = (r * sweep).hypot(rise);
        let sign = if anticlockwise { 1. } else { -1. };
        self.take(Screw { origin: c, axis: [0., 0., 1.], ratio: sign * sweep / length,
                          advance: rise / length * TAU }, length);
        self
    }

    /// A turn of the cutter, carried rigidly, about the line through `origin` along the unit
    /// `axis` by `angle` (radians, right-handed).
    pub fn turn(&mut self, origin: V, axis: V, angle: f64) -> &mut Self {
        let off = sub(self.tip(), origin);
        let along = scale(axis, off[0] * axis[0] + off[1] * axis[1] + off[2] * axis[2]);
        let radius = norm(sub(off, along)).max(1.);
        let length = radius * angle.abs();
        self.take(Screw { origin, axis, ratio: angle / length, advance: 0. }, length);
        self
    }

    /// The program's path of pieces and its roll, checked by the kernel to meet itself at each
    /// join to `tolerance`, over a reach of `reach`.
    pub fn path(self, tolerance: f64, reach: f64) -> Result<(Trajectory, [f64; 2]), String> {
        let end = self.roll;
        Trajectory::pieces(self.pieces, tolerance, reach)
            .map(|p| (p, [0., end]))
            .map_err(|e| format!("the program's path: {e:?}"))
    }
}

/// The closed corner path round the polygon `corners` (anticlockwise seen from above, in the plane
/// z = `z`), each corner rounded to `radius` by an arc tangent to both its sides: from the middle
/// of its first side, every side and every corner once, back to that middle.
pub fn rounded_loop(p: &mut Program, corners: &[[f64; 2]], radius: f64, z: f64) {
    let n = corners.len();
    let dir = |k: usize| {
        let (a, b) = (corners[k % n], corners[(k + 1) % n]);
        let d = [b[0] - a[0], b[1] - a[1], 0.];
        scale(d, 1. / norm(d))
    };
    for k in 1..=n {
        let v = [corners[k % n][0], corners[k % n][1], z];
        let (din, dout) = (dir(k - 1), dir(k));
        let left = cross(din, dout)[2] > 0.;
        let side = if left { [-din[1], din[0], 0.] } else { [din[1], -din[0], 0.] };
        let a = sub(v, scale(din, radius));
        let b = add(v, scale(dout, radius));
        let c = add(a, scale(side, radius));
        p.line(a);
        p.arc([c[0], c[1]], b, left, 0);
    }
    let first = corners[0];
    let second = corners[1 % n];
    let mid = scale(add([first[0], first[1], z], [second[0], second[1], z]), 0.5);
    p.line(mid);
}
