//! The box from (0, 0, 0) to (10, 6, 4) slid without turning along (8, 5, 0), level: a prism 4
//! high over the hexagon its floor sweeps.
use crate::sketch::block;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::slid;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{norm, unit};
use std::f64::consts::TAU;

const SIZE: V = [10., 6., 4.];
const SLIDE: V = [8., 5., 0.];

pub fn build() -> Result<Brep, String> {
    let tool = block([0., 0., 0.], SIZE)?;
    // the slide along (8, 5, 0): a screw that does not turn, advancing its whole length over a
    // roll of τ
    let path = Trajectory::Twist(Screw {
        origin: [0., 0., 0.],
        axis: unit(SLIDE),
        ratio: 0.,
        advance: norm(SLIDE),
    });
    // the kernel's closed swept solid (a box's flat faces mesh exactly, at any bar)
    let size = tool.size();
    slid(&tool, &path, [0., TAU], size, tolerance(size))
        .map_err(|why| format!("the box's sweep not built: {why:?}"))
}
