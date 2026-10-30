//! A cube of side 10 at the origin, its edge where x = 10 meets y = 10 rounded with a radius
//! growing along it, r = 1 + z/10.
//!
//! Refused. cad-kernel's rounds roll one ball: `fillet::roll(&b, at, r, tol)` takes a single radius
//! `r` for the whole run, and its canal is the ball of that radius swept along the offsets'
//! meeting.
//! Nothing in `fillet`, `corners` or a recipe's lowered round takes a radius that varies along the
//! edge (the meridian rounds of `corners` are constant too).
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no variable-radius round: fillet::roll rolls a ball of one radius along the whole edge, \
         and no operation of cad-kernel takes a radius that changes along it (r = 1 + z/10)"
        .into())
}
