//! The solid a ball of radius 2 fills as its center runs two turns of the helix of radius 1.5
//! about the z axis rising 3 a turn, from (1.5, 0, 0).
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // sweep::solid(ball, Motion::Screw) builds it, but the solid it makes has a face whose loops
    // bound no region of its surface, where the helix turns tighter than the ball
    Err("springrep's sweep of a ball along a helix tighter than the ball makes a face whose loops bound no region of its surface".into())
}
