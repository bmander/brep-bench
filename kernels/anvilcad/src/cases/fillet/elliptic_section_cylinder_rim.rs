//! A cylinder's top rim rounded with a quarter-ellipse section, 1 across the top and 3 down the side.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's rounds set back equally on both faces (a circular or conic section); it has no round of two different setbacks".into())
}
