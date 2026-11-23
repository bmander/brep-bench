//! A block, the box (−6, −6, −8)–(36, 26, 0), with a groove cut into its top by a square key, the
//! box (−1.5, −0.8, −1.5)–(1.5, 0.8, 2.5) drawn in a frame that rides along the cubic Bézier curve
//! (0, 0, 0), (10, 0, 0), (20, 20, 0), (30, 20, 0) — its x axis the curve's tangent, its z axis up
//! — and carried rigidly by it over the whole curve.
//!
//! The riding frame is the kernel's own frame carried along a curve (`trajectory::Along`, the
//! rotation-minimising one: along a curve in the plane z = 0 it keeps z up and its x axis the
//! tangent, which at the curve's start is the world's x). The motion keeps the plane z = 0, so
//! `swept::closed::cut_from` takes the planar road (`swept::planar_solid`) and cuts the key's
//! sweep from the block.
use crate::sketch::block;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::nurbs::BSpline;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Along, Trajectory};
use std::sync::Arc;

const CONTROL: [V; 4] = [[0., 0., 0.], [10., 0., 0.], [20., 20., 0.], [30., 20., 0.]];

pub fn build() -> Result<Brep, String> {
    let stock = block([-6., -6., -8.], [36., 26., 0.])?;
    let key = block([-1.5, -0.8, -1.5], [1.5, 0.8, 2.5])?;
    // the curve as cad-kernel holds a Bézier segment: a cubic B-spline on clamped knots, run over
    // [0, 1]; the frame's home is its start, where the riding frame is the world's own
    let curve = BSpline::new(3, vec![0., 0., 0., 0., 1., 1., 1., 1.], CONTROL.to_vec())?;
    let path = Trajectory::General(Arc::new(Along::new(curve)?));
    let size = stock.size().max(key.size());
    cut_from(&stock, &key, &path, [0., 1.], key.size(), tolerance(size))
        .map_err(|why| format!("the key's sweep not built: {why:?}"))
}
