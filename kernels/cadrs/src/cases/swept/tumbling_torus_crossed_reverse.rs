//! The torus of `swept/tumbling-torus-crossed-forward` rolled the same way about the upright line,
//! but turned by +t about the second line, t from −100° to 100°.
//!
//! Cadrs's sweep of a solid with the rigid motion of the two turns.
use crate::shared::swept::crossed_torus;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    crossed_torus(k, 1.0)
}
