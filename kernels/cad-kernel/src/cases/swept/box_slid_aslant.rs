//! The solid the box (0, 0, 0)–(10, 6, 4) fills as it slides without turning along the vector
//! (8, 5, 3), to the box (8, 5, 3)–(18, 11, 7).
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::family::{family, Run, Sheet, Stations, Wall, NODES};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{add, scale};
use std::sync::Arc;

const SIZE: V = [10., 6., 4.];
const SLIDE: V = [8., 5., 3.];

/// The solid the box's face square to axis `k` on its far side sweeps as it slides: the face's
/// rectangle carried along the slide, a face family over the fraction slid, each side a line whose
/// ends run along the slide — its wall the parallelogram they span.
fn slid_face(k: usize) -> Result<Brep, String> {
    let (i, j) = ((k + 1) % 3, (k + 2) % 3);
    let corner = |a: f64, b: f64| {
        let mut p = [0.; 3];
        (p[k], p[i], p[j]) = (SIZE[k], a * SIZE[i], b * SIZE[j]);
        p
    };
    let rectangle = [corner(0., 0.), corner(1., 0.), corner(1., 1.), corner(0., 1.)];
    let at = vec![NODES.map(|t| (1. + t) / 2.)];
    let stations = Arc::new(Stations::new(vec![[0., 1.]], at)?);
    let walls = (0..4).map(|e| {
        let nodes = stations.at.iter().flatten().map(|&s| {
            let step = scale(SLIDE, s);
            let (a, b) = (add(rectangle[e], step), add(rectangle[(e + 1) % 4], step));
            vec![a[0], a[1], a[2], b[0], b[1], b[2]]
        }).collect();
        let sheet = Sheet::new(Run::Line, stations.clone(), nodes, false)?;
        Ok(Wall { sheet: Arc::new(sheet), name: "slid".into() })
    }).collect::<Result<Vec<_>, String>>()?;
    family(&[walls], 0., None)
}

pub fn build() -> Result<Brep, String> {
    // workaround: no sweep of a solid with ends
    // cad-kernel has no sweep of a solid, but it sweeps a face along a line (a face family), so
    // the box's sweep is its faces' sweeps joined. A point passed through is p + s·(8, 5, 3) for p
    // in the box and s in [0, 1]: the segment from p either stays in the box or leaves it through
    // one of the faces the slide points out of — the far faces x = 10, y = 6 and z = 4, every
    // component of the slide being positive — and runs on in that face's sweep. So the solid is
    // the box where it starts united with those three faces each slid the whole way.
    let mut swept = block([0., 0., 0.], SIZE)?;
    for k in 0..3 {
        swept = combined(&swept, &slid_face(k)?, Op::Union, 0.)?;
    }
    Ok(swept)
}
