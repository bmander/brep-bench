//! A transition piece from the square −20 ≤ x, y ≤ 20 at z = 0 to the circle of radius 15 at
//! z = 30: flat triangles from the square's sides, oblique cones from its corners.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's ruled loft matches the square's sides to quarters of the circle by arc length; it pairs no corner with an arc, so it makes no oblique cone from a corner nor flat triangle from a side".into())
}
