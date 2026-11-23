//! The regularized difference of two independently swept tori, not of synchronized poses.
//!
//! Refused while its operands are: the forward and reverse sweeps (`tumbling_torus_crossed_forward`,
//! `tumbling_torus_crossed_reverse`) are refused at run time by the kernel's ring road
//! (`swept::closed::ring`, #942), so the difference is never asked.
use super::{tumbling_torus_crossed_forward, tumbling_torus_crossed_reverse};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let a = tumbling_torus_crossed_forward::build().map_err(|e| format!("forward operand: {e}"))?;
    let b = tumbling_torus_crossed_reverse::build().map_err(|e| format!("reverse operand: {e}"))?;
    combined(&a, &b, Op::Cut, 0.).map_err(|e| format!("swept operands' difference: {e}"))
}
