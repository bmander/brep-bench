//! The regularized intersection of two independently swept tori, not of synchronized poses.
//!
//! Asked at the tolerance its operands are built to (`Tumble::tolerance`, a fifty-thousandth of
//! the tube's radius), as `ring_pushed_in_at_a_lean` asks its cut at its sweep's: at the kernel's
//! own, a billionth of the size, the arrangement would resolve its sheets some five hundred times
//! finer than they are fitted.
//!
//! The two operands depend on nothing of each other's, so they are swept side by side, each on a
//! thread of its own.
use super::{tumbling_torus_crossed_forward, tumbling_torus_crossed_reverse};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let (a, b) = std::thread::scope(|s| {
        let reverse = s.spawn(tumbling_torus_crossed_reverse::build);
        let a = tumbling_torus_crossed_forward::build();
        (a, reverse.join().unwrap_or_else(|e| std::panic::resume_unwind(e)))
    });
    let a = a.map_err(|e| format!("forward operand: {e}"))?;
    let b = b.map_err(|e| format!("reverse operand: {e}"))?;
    // at the tolerance the two are built to: no finer than what it combines
    let tol = tumbling_torus_crossed_forward::tumble().tolerance();
    combined(&a, &b, Op::Common, tol).map_err(|e| format!("swept operands' intersection: {e}"))
}
