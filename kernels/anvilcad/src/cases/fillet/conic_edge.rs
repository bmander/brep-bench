//! The block from the origin to (20, 10, 10), the edge where its top meets its end x = 20 blended
//! with a conic of rho 0.5 (a parabola) tangent to both faces 3 from the edge.
use anvil_brep::Solid;
use anvil_brep::blend::{Blend, Profile};

use crate::shared::{blend, block};

pub fn build() -> Result<Solid, String> {
    // a profiled round touches the faces where a ball of its radius would: 3 from a square edge
    let parabola = Blend::Profiled { radius: 3., profile: Profile::ConicRho(0.5) };
    blend(&block([0., 0., 0.], [20., 10., 10.])?, &[[20., 5., 10.]], parabola)
}
