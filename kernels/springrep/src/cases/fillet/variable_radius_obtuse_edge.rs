//! A regular hexagonal prism of side 6 from z = 0 to 10, its edge through (6, 0) rounded with a
//! radius growing from 1 at the bottom to 4 at the top.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no variable-radius round of circular section: fillet_variable rolls a ball whose radius varies, and the envelope of its balls is a different round".into())
}
