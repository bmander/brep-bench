//! A cube of side 10 from the origin, the edge where its faces x = 10 and y = 10 meet rounded with
//! a radius growing evenly from 1 at the bottom to 2 at the top.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no variable-radius round of circular section: fillet_variable rolls a ball whose radius varies, and the envelope of its balls is a different round".into())
}
