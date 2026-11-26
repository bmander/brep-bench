//! What the cnc implementations share: an upright tool, and its moves along a level path the pen
//! draws, each swept by kentron's sweep of a solid (`sweep::solid`).
//!
//! A tool is built with its tip at the origin and its axis up, as the solid of revolution of its
//! half section (out from its axis, drawn in the xz plane). A straight move slides it; an arc
//! about an upright line carries its axis round that line without turning it, which, the tool
//! being a solid of revolution about its own axis, puts it where turning it about that line
//! does, so the arc is a turn (`RigidMotion::Turn`). kentron's sweep refuses a whole turn (its
//! ends would be one), so a whole circle is run as two half circles.
use kentron::math::{Axis, Direction, Point, Transform, Transformable};
use kentron::modify::{self, TransformOptions};
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

/// An upright tool, its tip at the origin: its half section `half`, drawn by the pen in the xz
/// plane (x out from the axis, y up), turned whole about the z axis.
pub fn upright(m: &mut Model, half: &[Step]) -> kentron::Result<Shape> {
    let face = sketch::profile(m, crate::shared::parts::xz_plane(0.0), half, Outcome::Face)?;
    Ok(sweep::revolve(m, face, Axis::Z, TAU)?.shape)
}

/// A move of a tool: where its tip starts, and the motion from there.
pub struct Move {
    pub from: Point,
    pub motion: RigidMotion,
}

/// A straight move of the tip from `from` to `to`.
pub fn line(from: Point, to: Point) -> Move {
    Move { from, motion: RigidMotion::Slide { by: to - from } }
}

/// The moves of the tip along the level path the pen `steps` draw on the plane z = `z` (from its
/// `Start`, with `Forward`, `Heading` and `Arc` steps): a slide for each line, a turn about the
/// upright line through its centre for each arc (a whole circle in two halves).
pub fn along(steps: &[Step], z: f64) -> Vec<Move> {
    let (mut at, mut heading) = ((0.0, 0.0), 0f64);
    let mut moves = Vec::new();
    for step in steps {
        match *step {
            Step::Start(x, y) => at = (x, y),
            Step::Heading(degrees) => heading = degrees.to_radians(),
            Step::Forward(length) => {
                let to = (at.0 + length * heading.cos(), at.1 + length * heading.sin());
                moves.push(line(Point::new(at.0, at.1, z), Point::new(to.0, to.1, z)));
                at = to;
            }
            Step::Arc { radius, angle } => {
                // The centre on the left for a positive radius; turning anticlockwise then.
                let centre = (at.0 - radius * heading.sin(), at.1 + radius * heading.cos());
                let turn = angle.to_radians() * radius.signum();
                let pieces = if turn.abs() >= TAU - 1e-12 { 2 } else { 1 };
                let axis = Axis::new(Point::new(centre.0, centre.1, 0.0), Direction::Z);
                for _ in 0..pieces {
                    let motion = RigidMotion::Turn { axis, from: 0.0, to: turn / pieces as f64 };
                    let from = Point::new(at.0, at.1, z);
                    let to = from.transformed(&motion.at(1.0));
                    moves.push(Move { from, motion });
                    at = (to.x, to.y);
                }
                heading += turn;
            }
            ref other => panic!("a tool path step not run here: {other:?}"),
        }
    }
    moves
}

/// What the tool `tool` builds (its tip at the origin, its axis up) passes through on each of
/// `moves`, kentron's sweep of a solid of the tool placed where the move starts; the moves swept
/// at once, each in a model of its own (`parallel::each`).
pub fn swept(m: &mut Model, tool: impl Fn(&mut Model) -> kentron::Result<Shape> + Sync, moves: &[Move]) -> kentron::Result<Vec<Shape>> {
    kentron::parallel::each(m, moves.len(), |m, k| {
        let Move { from, motion } = &moves[k];
        let tool = tool(m)?;
        let placed = modify::transform(m, tool, &Transform::translation(*from - Point::ORIGIN), TransformOptions::default())?;
        sweep::solid(m, placed, motion)
    })
}

/// `body` less everything `cuts` pass through, all together.
pub fn cut(m: &mut Model, body: Shape, cuts: &[Shape]) -> kentron::Result<Shape> {
    let all = kentron::boolean::union_all(m, cuts)?;
    kentron::boolean::difference(m, body, all)
}

