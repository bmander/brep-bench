//! An L-shaped block 10 deep, its inside edge rounded with a radius growing from 1 at y = 0 to 3
//! at y = 10, adding material.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no variable-radius round of circular section: fillet_variable rolls a ball whose radius varies, and the envelope of its balls is a different round".into())
}
