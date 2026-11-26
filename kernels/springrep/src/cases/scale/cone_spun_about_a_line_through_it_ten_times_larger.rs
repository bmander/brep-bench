//! `swept/cone-spun-about-a-line-through-it` ten times larger: the solid a cone (axis along
//! (cos 30°, 0, sin 30°), point at −20 along it, opening at 50° to a base at 50) fills as it turns
//! 150° about the z axis. Built as the original is, every length times 10.
use springrep::math::Frame;
use springrep::primitive::cone;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, turning, v3};

pub fn build() -> Result<Brep, String> {
    let (s, c) = deg(30.0).sin_cos();
    let w = v3(c, 0.0, s);
    let point = Frame::from_axis(w * -20.0, w).map_err(e)?;
    let tool = cone(&point, 0.0, 70.0 * deg(25.0).tan(), 70.0, "cone").map_err(e)?;
    solid(&tool, &turning(0.0, deg(150.0)), "spin").map_err(e)
}
