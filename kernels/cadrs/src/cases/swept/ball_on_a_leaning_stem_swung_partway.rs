//! A curved block (the ring sector between radii 8 and 12, heights ±1, polar angles 0° to 60°)
//! notched by a ball (radius 0.8 about (10, 0, 0)) on a stem (a cylinder of radius 0.4 from the
//! ball's centre 3 along (0, sin 20°, cos 20°)) turned about the z axis from −140° to +30°.
//!
//! The tool is the ball and the stem united; what it passes through (one turn) is cut from the
//! block (cadrs's cylindrical box) by cadrs's cut by a swept solid (the exact sweep cut from it
//! where it can be, else meshed).
use crate::bench::{ball, cylinder, p3, union, v3};
use crate::shared::swept::{cut_swept, ring_sector, swing, z_axis};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let block = ring_sector(k, (8.0, 12.0), (-1.0, 1.0), (0.0, 60.0))?;
    let a = 20f64.to_radians();
    let b = ball(k, p3(10.0, 0.0, 0.0), 0.8)?;
    let stem = cylinder(k, p3(10.0, 0.0, 0.0), v3(0.0, a.sin(), a.cos()), 0.4, 3.0)?;
    let tool = union(k, b, stem)?;
    cut_swept(k, block, tool, &swing(z_axis(), -140.0, 30.0))
}
