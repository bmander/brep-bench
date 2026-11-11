//! What the cadrs implementations of the shell cases share: cadrs's Shell feature
//! (`Kernel::shell_with`, Onshape's dialog), its openings picked as a user picks faces in the
//! viewport, by a point on each.
use crate::bench::P3;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Kernel, Pick, ShellSpec};

/// How the walls of a shell are joined.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Joins {
    /// Rounded (cadrs's default, OCCT's arc join): outward, a cylinder round each convex edge
    /// and a ball at each corner; inward, the cavity rounded at the body's concave edges.
    Rounded,
    /// Sharp: the moved faces extended until they meet.
    Sharp,
}

/// The body that `spec` makes of `body`: one body, the result's first.
fn run(k: &mut DefaultKernel, body: BodyId, spec: ShellSpec, joins: Joins) -> Result<BodyId, Refusal> {
    let spec = if joins == Joins::Sharp { spec.mitred() } else { spec };
    Ok(k.shell_with(body, &spec)?.first()?)
}

/// `body` shelled inward to walls `thickness` thick, open at the faces through `openings`.
pub fn shell_inward(
    k: &mut DefaultKernel,
    body: BodyId,
    openings: &[P3],
    thickness: f64,
    joins: Joins,
) -> Result<BodyId, Refusal> {
    let remove = k.faces_at(body, openings)?;
    run(k, body, ShellSpec::inward(remove, thickness), joins)
}

/// `body` shelled outward to walls `thickness` thick, open at the faces through `openings`.
pub fn shell_outward(
    k: &mut DefaultKernel,
    body: BodyId,
    openings: &[P3],
    thickness: f64,
    joins: Joins,
) -> Result<BodyId, Refusal> {
    let remove = k.faces_at(body, openings)?;
    run(k, body, ShellSpec::outward(remove, thickness), joins)
}

/// `body` hollowed to walls `thickness` thick with no opening: the inside a closed void.
pub fn hollow(k: &mut DefaultKernel, body: BodyId, thickness: f64) -> Result<BodyId, Refusal> {
    run(k, body, ShellSpec::hollow(thickness), Joins::Rounded)
}
