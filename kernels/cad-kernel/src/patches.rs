//! What the patch cases share: the block under a Bézier patch whose top face is trimmed on the
//! whole patch, its walls upright and its floor at z = 0 (`nurbs/folded-patch-notched-at-its-rim`
//! and the three beside it).
use cad_kernel::build::sheet;
use cad_kernel::geom::{tolerance, Surface};
use cad_kernel::nurbs::Net;
use cad_kernel::sew::sewn;
use cad_kernel::topo::Brep;
use std::sync::Arc;

/// The Bézier net on `poles` (row i along u, column j along v): degree one less than the count
/// each way, its knots clamped on [0, 1].
pub fn bezier(poles: Vec<Vec<[f64; 3]>>) -> Net {
    let clamped = |d: usize| [vec![0.; d + 1], vec![1.; d + 1]].concat();
    let (du, dv) = (poles.len() - 1, poles[0].len() - 1);
    Net {
        du,
        dv,
        uknots: clamped(du),
        vknots: clamped(dv),
        poles,
        weights: None,
    }
}

/// The block on the floor z = 0 under the part `us` × `vs` of `patch` (a graph over the floor),
/// its walls upright, sewn: its top face on the whole of `patch`, trimmed to that part. The faces
/// are built on the part (`Net::segment`, the same surface at the same parameters, its rows and
/// columns of poles the walls' rims exactly) and sewn; the top then stands on the whole patch, its
/// trims, in the same parameters, as they were.
pub fn under(patch: &Net, us: [f64; 2], vs: [f64; 2]) -> Result<Brep, String> {
    let top = patch.segment(us, vs);
    let floor = Net {
        poles: top
            .poles
            .iter()
            .map(|r| r.iter().map(|p| [p[0], p[1], 0.]).collect())
            .collect(),
        ..top.clone()
    };
    // a wall under a rim of the top: degree 1 up from its foot on the floor
    let wall = |rim: Vec<[f64; 3]>, knots: &[f64], d: usize| Net {
        du: d,
        dv: 1,
        uknots: knots.to_vec(),
        vknots: vec![0., 0., 1., 1.],
        poles: rim.iter().map(|p| vec![[p[0], p[1], 0.], *p]).collect(),
        weights: None,
    };
    let (last, lastv) = (top.poles.len() - 1, top.poles[0].len() - 1);
    let walls = [
        wall(top.poles[0].clone(), &top.vknots, top.dv),
        wall(top.poles[last].clone(), &top.vknots, top.dv),
        wall(
            top.poles.iter().map(|r| r[0]).collect(),
            &top.uknots,
            top.du,
        ),
        wall(
            top.poles.iter().map(|r| r[lastv]).collect(),
            &top.uknots,
            top.du,
        ),
    ];
    let mut pieces = vec![sheet(top.clone())?, sheet(floor)?];
    pieces[0].faces[0].name = "top".into();
    for w in walls {
        pieces.push(sheet(w)?);
    }
    let mut block = sewn(&pieces, tolerance(2.))?;
    let face = block
        .faces
        .iter_mut()
        .find(|f| f.name == "top")
        .ok_or("the top face sewn")?;
    let Surface::BSpline(frame, _) = face.surface else {
        return Err("the top not on its patch".into());
    };
    face.surface = Surface::BSpline(frame, Arc::new(patch.clone()));
    Ok(block)
}
