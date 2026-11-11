//! The gear ladder's solids (`swept/*-cut-by-its-pinion`, `swept/*-hobbed-by-its-worm`): the bevel
//! blank and its pinion's tooth, the worm's thread, and the pair's two turns as the blank sees them.
use crate::sketch::polygon;
use cad_kernel::build::{kept, prism, revolve, screwed};
use cad_kernel::geom::{Rigid, V};
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{add, cross, dot, scale, unit};
use std::f64::consts::{FRAC_PI_2, TAU};

/// The bevel pair's frame: g along the line where the pitch cones touch, d square to it in the
/// xz-plane, from the pinion into the gear.
fn bevel_frame() -> (V, V) {
    let s5 = 5f64.sqrt();
    ([2. / s5, 0., 1. / s5], [-1. / s5, 0., 2. / s5])
}

/// The bevel gear's blank: the quadrilateral of the points s·g + u·d (s from 15√5 − 10 to 15√5, u
/// from the face cone through O, 2 outside the pitch cone at the outer end, to 6 inside it) in the
/// xz-plane, revolved about the z axis.
pub fn bevel_blank() -> Result<Brep, String> {
    let (g, d) = bevel_frame();
    let (ae, ai) = (15. * 5f64.sqrt(), 15. * 5f64.sqrt() - 10.);
    let at = |s: f64, u: f64| add(scale(g, s), scale(d, u));
    let quad = polygon(&[at(ae, -2.), at(ae, 6.), at(ai, 6.), at(ai, -2. * ai / ae)]);
    revolve(&quad, [0.; 3], [0., 0., 1.], TAU)
}

/// The pinion's tooth: the pyramid with apex O over the trapezoid P + u·d + w·y (P = 15√5·g,
/// |u| ≤ 2.5, |w| ≤ π/2 − u·tan 20°), between the planes square to g at 15√5 − 12 and 15√5 + 2.
/// A prism along g between those planes on a wider trapezoid, kept behind each of the pyramid's
/// four flank planes through O.
pub fn bevel_tooth() -> Result<Brep, String> {
    let (g, d) = bevel_frame();
    let ae = 15. * 5f64.sqrt();
    let (inner, outer) = (ae - 12., ae + 2.);
    let y = [0., 1., 0.];
    let w = |u: f64| FRAC_PI_2 - u * 20f64.to_radians().tan();
    // the trapezoid's corners at the outer pitch point, walked so that they face +g
    let corners: Vec<V> = [(2.5, -w(2.5)), (-2.5, -w(-2.5)), (-2.5, w(-2.5)), (2.5, w(2.5))]
        .iter()
        .map(|&(u, w)| add(add(scale(g, ae), scale(d, u)), scale(y, w)))
        .collect();
    // the prism: the trapezoid scaled from the outer pitch point by 1.2, out to the outer plane,
    // then moved in to the inner one and carried out along g
    let wide: Vec<V> = corners
        .iter()
        .map(|&c| {
            let k = outer / ae;
            let c = scale(add(scale(g, ae), scale(add(c, scale(g, -ae)), 1.2)), k);
            add(c, scale(g, inner - dot(c, g)))
        })
        .collect();
    let section = polygon(&wide);
    let mut tooth = if dot(section.normal, g) > 0. {
        prism(&section, 0., outer - inner)?
    } else {
        prism(&section, -(outer - inner), 0.)?
    };
    // each flank: the plane through O and two neighbouring corners, the tooth kept on its middle's side
    let middle = scale(g, ae);
    for k in 0..4 {
        let n = unit(cross(corners[k], corners[(k + 1) % 4]));
        let keep = if dot(n, middle) > 0. { n } else { scale(n, -1.) };
        tooth = kept(&tooth, [0.; 3], keep, 0.)?;
    }
    Ok(tooth)
}

/// A turn at `ratio` radians a radian about the line through `origin` along `axis`.
pub fn turn(origin: V, axis: V, ratio: f64) -> Trajectory {
    Trajectory::Twist(Screw { origin, axis, ratio, advance: 0. })
}

/// The tool turning at 1 about its axis seen from the gear turning at `ratio` about the z axis.
pub fn in_mesh(tool: Trajectory, ratio: f64) -> Trajectory {
    Trajectory::Relative { source: Box::new(tool), observer: Box::new(turn([0.; 3], [0., 0., 1.], ratio)) }
}

/// The cylindrical worm's thread on the line through `at` along the x axis, from x = `from` to `to`.
/// Built about the z axis: its axial section, the trapezoid π wide at radius 10 from radius 7.5 to
/// 12.5, its sides 20° from square to the axis, in the half-plane y = 0, x > 0, screwed a turn past
/// each end at a lead of 2π, right-handed, and cut to the planes z = `from` and `to`; then turned
/// by 90° about the y axis (z onto x, the half-plane onto −z, a quarter turn behind +y about the
/// new axis, so the section is centred on z ≡ −π/2 there) and moved to `at`.
pub fn worm_thread(at: V, from: f64, to: f64) -> Result<Brep, String> {
    let w = |r: f64| FRAC_PI_2 - (r - 10.) * 20f64.to_radians().tan();
    let before = (-from / TAU).ceil() + 1.;
    let z0 = -TAU * before - FRAC_PI_2;
    let section = polygon(&[
        [7.5, 0., z0 - w(7.5)],
        [12.5, 0., z0 - w(12.5)],
        [12.5, 0., z0 + w(12.5)],
        [7.5, 0., z0 + w(7.5)],
    ]);
    let turns = ((to - z0) / TAU).ceil() + 1.;
    let thread = screwed(&section, [0.; 3], [0., 0., 1.], TAU * turns, TAU * turns)?;
    let thread = kept(&thread, [0., 0., from], [0., 0., 1.], 0.)?;
    let thread = kept(&thread, [0., 0., to], [0., 0., -1.], 0.)?;
    let onto = Rigid::turn([0.; 3], [0., 1., 0.], FRAC_PI_2).then(&Rigid { t: at, ..Rigid::identity() });
    Ok(thread.moved(&onto))
}

/// The blank `blank` less what the worm's thread on the line through `at` along x (from x = `from`
/// to `to`) passes through as it turns by t about its axis while the gear turns by t/30 about the z
/// axis, t from 0 to 60π: the kernel's arrangement cutting the sweep from the blank
/// (`swept::closed::cut_from`).
pub fn hobbed(blank: &Brep, at: V, from: f64, to: f64) -> Result<Brep, String> {
    let thread = worm_thread(at, from, to)?;
    let path = in_mesh(turn(at, [1., 0., 0.], 1.), 1. / 30.);
    cad_kernel::swept::closed::cut_from(blank, &thread, &path, [0., 60. * std::f64::consts::PI], 0.01,
        cad_kernel::geom::tolerance(blank.size()))
        .map_err(|why| format!("the worm's sweep not cut: {why:?}"))
}
