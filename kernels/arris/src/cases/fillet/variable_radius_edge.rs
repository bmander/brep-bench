//! A cube of side 10 at the origin, the edge where its faces x = 10 and y = 10 meet rounded with a
//! radius growing linearly along it, 1 at z = 0 and 2 at z = 10.
//!
//! Refused: arris's `fillet` rolls a ball of one radius, the same along every edge it is given,
//! and arris has no blend whose radius varies along its edge.
use crate::Refusal;
use arris::topo::{Body, Model};

pub fn build(_: &mut Model) -> Result<Body, Refusal> {
    Err("arris has no variable-radius fillet: `fillet` rolls a ball of one constant radius".into())
}
