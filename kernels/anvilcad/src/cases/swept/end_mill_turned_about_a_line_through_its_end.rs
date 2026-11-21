//! A pocket cut by an end mill turned about a line through its flat end.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no sweep of a solid, and what this motion sweeps is bounded by envelopes of the body's surfaces under it (a rim or a face leaning to the motion) that follow from no elementary construction, which it would have to compute itself".into())
}
