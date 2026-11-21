//! An oblique torus tumble through a full turn.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no sweep of a solid: a body tumbled by two turns at once sweeps envelopes of its surfaces that follow from no elementary construction, which it would have to compute itself".into())
}
