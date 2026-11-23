//! The drum of radius 5 on the z axis from z = 0 to 10 united with a sleeve round its middle: every
//! point between 5 and 7 from the axis with 2 ≤ z ≤ 8, its outer side a cylinder, its ends flat
//! annuli, and its bore a rational B-spline surface lying exactly on the drum's cylinder (the
//! rational quadratic circle of nine control points from (0, 5) round to it, degree 1 along z).
//!
//! Refused at run time by cad-kernel 0.21.0's Boolean: the sleeve is built and sewn shut (its
//! volume 144π, valid), but the union does not recognise the B-spline bore as lying on the drum's
//! cylinder and meets the two as surfaces crossing: "a cylinder and a bspline meet at 0.00° at
//! [0.2980, −4.9911, 2.5000], nearly touching: their meeting is ill-conditioned there, not built
//! yet" (`ssi`). The operands the other way round, and `boolean::combined_written`, are refused
//! the same way.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, face, grid_sheet, prism, tube, Profile, ProfileEdge};
use cad_kernel::geom::{tolerance, Frame};
use cad_kernel::nurbs::Net;
use cad_kernel::sew::sewn;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;
use std::sync::Arc;

const DRUM_RADIUS: f64 = 5.;
const DRUM_HEIGHT: f64 = 10.;
const OUTER: f64 = 7.;
const ENDS: [f64; 2] = [2., 8.];
const H: f64 = FRAC_1_SQRT_2;
/// The bore's circle in (x, y), from (0, 5) round to it, and its weights.
const BORE: [[f64; 2]; 9] = [
    [0., 5.], [-5., 5.], [-5., 0.], [-5., -5.], [0., -5.], [5., -5.], [5., 0.], [5., 5.], [0., 5.],
];
const BORE_W: [f64; 9] = [1., H, 1., H, 1., H, 1., H, 1.];

/// The sleeve: its bore the net, its outer side a cylinder, its ends annuli, sewn shut.
fn sleeve() -> Result<Brep, String> {
    let [z0, z1] = ENDS;
    let net = Net {
        du: 2,
        dv: 1,
        uknots: vec![0., 0., 0., 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1., 1., 1.],
        vknots: vec![0., 0., 1., 1.],
        poles: BORE.iter().map(|&[x, y]| vec![[x, y, z0], [x, y, z1]]).collect(),
        weights: Some(BORE_W.iter().map(|&w| vec![w, w]).collect()),
    };
    let names = vec!["bore".to_string(), "bore".to_string()];
    let bore = grid_sheet(&net, &[0., 0.5, 1.], &[0., 1.], &names, [true, false])?;
    let up = [0., 0., 1.];
    let outer = tube([0., 0., z0], up, OUTER, z1 - z0, 2)?;
    // each end: the disc of radius 7 less the bore's own rim there (the net's edge at that z)
    let end = |z: f64, normal: [f64; 3], j: usize| face(&Profile {
        origin: [0., 0., z],
        normal,
        loops: vec![
            vec![ProfileEdge::Arc { frame: Frame::about([0., 0., z], up), r: OUTER, span: None }],
            vec![ProfileEdge::Spline(Arc::new(net.column(j)))],
        ],
        names: Vec::new(),
    });
    let ends = [end(z0, [0., 0., -1.], 0)?, end(z1, up, 1)?];
    Ok(sewn(&[bore, outer, ends[0].clone(), ends[1].clone()], tolerance(2. * OUTER))?)
}

pub fn build() -> Result<Brep, String> {
    let drum = prism(&disc([0., 0., 0.], [0., 0., 1.], DRUM_RADIUS), 0., DRUM_HEIGHT)?;
    // the union must find the bore lying on the drum's side
    Ok(combined(&drum, &sleeve()?, Op::Union, 0.)?)
}
