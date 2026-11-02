//! The cylinder of radius 10 on the z axis from z = 0 to z = 10, a groove round its side from
//! z = 4.2 to z = 5.8 cut to radius 7, grown by 1.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, Profile, ProfileEdge};
use cad_kernel::geom::Frame;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let drum = prism(&disc([0., 0., 0.], [0., 0., 1.], 10.), 0., 10.)?;
    // the groove: the ring from radius 7 out past the side
    let o = [0., 0., 4.2];
    let ring = Profile { origin: o, normal: [0., 0., 1.], names: Vec::new(), loops: vec![
        vec![ProfileEdge::Arc { frame: Frame::about(o, [0., 0., 1.]), r: 12., span: None }],
        vec![ProfileEdge::Arc { frame: Frame::about(o, [0., 0., 1.]), r: 7., span: None }],
    ] };
    let groove = prism(&ring, 0., 1.6)?;
    grown(&combined(&drum, &groove, Op::Cut, 0.)?, 1., Join::Round)
}
