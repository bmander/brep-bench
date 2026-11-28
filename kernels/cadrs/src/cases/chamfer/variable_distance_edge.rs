//! A cube of side 10 between the origin and (10, 10, 10), the edge where its faces x = 10 and
//! y = 10 meet chamfered with a distance growing along it on the face x = 10, d₁ = 1 + z/5, and a
//! constant 2 on the face y = 10: the chamfer the hyperbolic paraboloid ruled by the horizontal
//! lines from (10, 10 − d₁, z) to (8, 10, z).
//!
//! cadrs's variable chamfer (`Kernel::chamfer_variable`): its distance laws run from the edge's
//! start to its end, so each end gets the distance at its height.
use crate::bench::{block, p3};
use crate::Refusal;
use cadrs_kernel::{BodyId, ChamferLaw, DefaultKernel, Kernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let cube = block(k, p3(0.0, 0.0, 0.0), p3(10.0, 10.0, 10.0))?;
    let edge = k.edge_at(cube, p3(10.0, 10.0, 5.0))?;
    let side = k.face_at(cube, p3(10.0, 5.0, 5.0))?;
    let info = k.edges(cube)?.into_iter().find(|e| e.id == edge).ok_or("the edge is gone")?;
    // d₁ = 1 + z/5 from the edge's start to its end
    let d1 = |z: f64| 1.0 + z / 5.0;
    let law = ChamferLaw {
        edge,
        face: Some(side),
        first: vec![(0.0, d1(info.start.z)), (1.0, d1(info.end.z))],
        second: vec![(0.0, 2.0)],
    };
    Ok(k.chamfer_variable(cube, &[law])?.first()?)
}
