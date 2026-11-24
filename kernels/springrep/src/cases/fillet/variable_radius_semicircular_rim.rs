//! A half cylinder of radius 5 from z = 0 to 6 on the side x ≥ 0, the half circle where its side
//! meets its top rounded with a radius growing linearly with the angle from 1 to 2.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no variable-radius round of circular section: fillet_variable rolls a ball whose radius varies, and the envelope of its balls is a different round".into())
}
