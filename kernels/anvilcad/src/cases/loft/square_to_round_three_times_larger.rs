//! A square-to-round transition three times larger.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's ruled loft matches the square's sides to quarters of the circle by arc length; it pairs no corner with an arc, so it makes no oblique cone from a corner nor flat triangle from a side".into())
}
