//! A cylinder of radius 5 on the z axis from z = 0 to 10, its top rim blended with a conic of rho
//! 0.5 (a parabola) tangent to the top and the side 2 from the edge.
use anvil_brep::blend::{Blend, Profile};
use anvil_brep::{Frame, Solid};

use crate::shared::{blend, cylinder, ring};

pub fn build() -> Result<Solid, String> {
    // a profiled round touches the faces where a ball of its radius would: 2 from a square edge
    let parabola = Blend::Profiled { radius: 2., profile: Profile::ConicRho(0.5) };
    blend(&cylinder(&Frame::world(), 5., 0., 10.)?, &[ring(5., 10.)], parabola)
}
