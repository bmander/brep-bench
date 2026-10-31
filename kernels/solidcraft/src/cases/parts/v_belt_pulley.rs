use crate::solids::*;

pub fn build() -> R<Body> {
    let hub = zcyl(0.0, 0.0, 12.0, -14.0, 14.0)?;
    let rim = zcyl(0.0, 0.0, 28.0, -9.0, 9.0)?;
    let pulley = cut(&union(&hub, &rim)?, &zcyl(0.0, 0.0, 6.0, -15.0, 15.0)?)?;
    let groove = revolve_z(polygon(&[(22.0, 0.0), (29.0, -5.0), (29.0, 5.0)]), 360.0)?;
    cut(&pulley, &groove)
}
