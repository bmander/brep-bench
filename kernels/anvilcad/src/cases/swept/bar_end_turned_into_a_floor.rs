//! A floor cut by a leaning bar's flat end, turned.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no sweep of a solid, and what this motion sweeps is bounded by envelopes of the body's surfaces under it (a rim or a face leaning to the motion) that follow from no elementary construction, which it would have to compute itself".into())
}
