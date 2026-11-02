//! A ball of radius 5 about the origin bored through along z with a hole of radius 3, grown by 1.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, sphere};
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let ball = sphere([0., 0., 0.], [0., 0., 1.], 5.)?;
    let bore = prism(&disc([0., 0., -6.], [0., 0., 1.], 3.), 0., 12.)?;
    grown(&combined(&ball, &bore, Op::Cut, 0.)?, 1., Join::Round)
}
