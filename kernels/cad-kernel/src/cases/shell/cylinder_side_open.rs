//! A cylinder of radius 5 from z = 0 to 10 shelled inward to a wall 1 thick, open at its curved
//! side: two discs.
use crate::sketch::faces_through;
use cad_kernel::build::{disc, prism};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = prism(&disc([0., 0., 0.], [0., 0., 1.], 5.), 0., 10.)?;
    let open = faces_through(&body, &[[0., 5., 5.]])?;
    shelled(&body, &open, 1., Side::In, Join::Round)
}
