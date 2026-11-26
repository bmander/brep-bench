//! The oblate spheroid (x² + y²)/36 + z²/9 ≤ 1 of `geometry/oblate-spheroid` hollowed to a wall 1
//! thick with no opening, as `shell/sphere-hollowed` hollows its ball: springrep's own shell.
use springrep::edit::Wall;
use springrep::topo::Brep;

use crate::cases::geometry::r#oblate_spheroid;
use crate::shared::shell::shelled;

pub fn build() -> Result<Brep, String> {
    shelled(&oblate_spheroid::build()?, &[], 1., Wall::Inward, "shell")
}
