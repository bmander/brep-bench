//! What the swept implementations share: pieces cut in turn, the tools and blocks several cases
//! use, and a frame carried along a curve.
//!
//! A swept solid is kentron's sweep of a solid (`sweep::solid`) of the tool under the motion the
//! case describes: a slide, a turn, a screw, or a motion given by its places (`Carried`).
use kentron::boolean;
use kentron::math::Point;
use kentron::topology::{Model, Shape};

/// `body` with each of `pieces` cut from it in turn, as holes are cut one after another: what
/// is left is `body` less their union, and pieces that only touch are never fused to each other.
pub fn cut_each(m: &mut Model, body: Shape, pieces: &[Shape]) -> kentron::Result<Shape> {
    let mut body = body;
    for &piece in pieces {
        body = boolean::difference(m, body, piece)?;
    }
    Ok(body)
}

/// The bull-nose mill drawn about the z axis from its flat end at the origin up 3 (a cylinder of
/// radius 2, its end corner rounded to 0.5: a disc of radius 1.5, a quarter torus, the side),
/// its meridian revolved, then moved by `place`.
pub fn bull_nose(m: &mut Model, place: &kentron::math::Transform) -> kentron::Result<Shape> {
    use kentron::geometry::curve::{Circle, Curve};
    use kentron::math::{Axis, Direction, Frame};
    let xz = |x: f64, z: f64| Point::new(x, 0.0, z);
    let round = Circle::new(Frame::new(xz(1.5, 0.5), Direction::new(0.0, -1.0, 0.0), Direction::X), 0.5);
    let edges = vec![
        kentron::make::edge_between(m, xz(0.0, 0.0), xz(1.5, 0.0))?,
        kentron::make::edge_from_curve_range(m, Curve::Circle(round), -std::f64::consts::FRAC_PI_2, 0.0)?,
        kentron::make::edge_between(m, xz(2.0, 0.5), xz(2.0, 3.0))?,
        kentron::make::edge_between(m, xz(2.0, 3.0), xz(0.0, 3.0))?,
        kentron::make::edge_between(m, xz(0.0, 3.0), xz(0.0, 0.0))?,
    ];
    let wire = kentron::make::wire(m, &edges)?;
    let meridian = kentron::make::planar_face(m, wire)?;
    let upright = kentron::sweep::revolve(m, meridian, Axis::Z, std::f64::consts::TAU)?.shape;
    kentron::modify::transform(m, upright, place, Default::default())
}

/// The sector of the ring about the z axis between radii 8 and 12, z from −1 to 1, polar angle
/// from 0° to 60°.
pub fn ring_sector(m: &mut Model) -> kentron::Result<Shape> {
    let z = kentron::math::Axis::new(Point::new(0.0, 0.0, 0.0), kentron::math::Direction::Z);
    let section = crate::shared::primitives::rectangle_xz(m, (8.0, -1.0), (12.0, 1.0))?;
    Ok(kentron::sweep::revolve(m, section, z, std::f64::consts::FRAC_PI_3)?.shape)
}

/// The frame riding the cubic Bézier curve through (0, 0, 0), (10, 0, 0), (20, 20, 0) and
/// (30, 20, 0) at `t`: its origin the curve's point, its x axis the tangent, its z axis up.
pub fn bezier_frame(t: f64) -> kentron::math::Transform {
    level_bezier_frame(&[(0.0, 0.0), (10.0, 0.0), (20.0, 20.0), (30.0, 20.0)], t)
}

/// The frame riding the level cubic Bézier curve on the control points `p` (in the plane z = 0)
/// at `t`: its origin the curve's point, its x axis the tangent, its z axis up.
pub fn level_bezier_frame(p: &[(f64, f64); 4], t: f64) -> kentron::math::Transform {
    let s = 1.0 - t;
    let at = |k: usize| s * s * s * [p[0].0, p[0].1][k] + 3.0 * s * s * t * [p[1].0, p[1].1][k] + 3.0 * s * t * t * [p[2].0, p[2].1][k] + t * t * t * [p[3].0, p[3].1][k];
    let along = |k: usize| 3.0 * (s * s * ([p[1].0, p[1].1][k] - [p[0].0, p[0].1][k]) + 2.0 * s * t * ([p[2].0, p[2].1][k] - [p[1].0, p[1].1][k]) + t * t * ([p[3].0, p[3].1][k] - [p[2].0, p[2].1][k]));
    let (dx, dy) = (along(0), along(1));
    let l = dx.hypot(dy);
    let (c, sn) = (dx / l, dy / l);
    kentron::math::Transform::from_rows([[c, -sn, 0.0, at(0)], [sn, c, 0.0, at(1)], [0.0, 0.0, 1.0, 0.0]]).expect("a rigid frame")
}

/// A tumble, as a motion given by its places (`sweep::RigidMotion::Carried`): turned by
/// `first_rate`·t about `first` and then by `second_rate`·t about `second`, for t from `from` to
/// `to` (degrees).
pub fn tumble(first: kentron::math::Axis, first_rate: f64, second: kentron::math::Axis, second_rate: f64, from: f64, to: f64) -> kentron::sweep::RigidMotion {
    use kentron::math::Transform;
    let place = move |s: f64| {
        let t = (from + (to - from) * s).to_radians();
        Transform::rotation(second, second_rate * t) * Transform::rotation(first, first_rate * t)
    };
    kentron::sweep::RigidMotion::Carried(kentron::sweep::Carriage(std::sync::Arc::new(place)))
}

/// What the torus of the crossed tumbles fills (`swept/tumbling-torus-crossed-forward` and
/// `-reverse`): centred at (3, 0, 2), axis parallel to the z axis, radii 1 and 0.22, turned by
/// 0.7t about the vertical line through (2, 0, 0) and then by `second_rate`·t about the line
/// through (0, h cos 45°, h sin 45°), h = 0.7071, along (cos 20°, sin 20° cos 45°, sin 20° sin 45°),
/// for t from −100° to 100°: kentron's sweep of a solid (`sweep::solid`) under that `tumble`.
pub fn crossed_tumble(m: &mut Model, second_rate: f64) -> kentron::Result<Shape> {
    use kentron::math::{Axis, Direction, Frame};
    let torus = kentron::primitives::Torus::new(1.0, 0.22).with_frame(Frame::from_z(Point::new(3.0, 0.0, 2.0), Direction::Z)).build(m)?;
    let (sa, ca) = 45f64.to_radians().sin_cos();
    let (s20, c20) = 20f64.to_radians().sin_cos();
    let second = Axis::new(Point::new(0.0, 0.7071 * ca, 0.7071 * sa), Direction::new(c20, s20 * ca, s20 * sa));
    let first = Axis::new(Point::new(2.0, 0.0, 0.0), Direction::Z);
    kentron::sweep::solid(m, torus, &tumble(first, 0.7, second, second_rate, -100.0, 100.0))
}
