//! A pin a tenth of a millimetre across, its rim rounded unequally by a quarter ellipse.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's rounds set back equally on both faces (a circular or conic section); it has no round of two different setbacks".into())
}
