//! A variant of `swept/ball-screwed-round-a-tight-helix` (scaled or rigidly turned; see the case):
//! the solid a ball fills as its center runs two turns of a helix tighter than the ball. Refused
//! for the original's reason.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // sweep::solid(ball, Motion::Screw) builds it, but the solid it makes has a face whose loops
    // bound no region of its surface, where the helix turns tighter than the ball
    Err("springrep's sweep of a ball along a helix tighter than the ball makes a face whose loops bound no region of its surface".into())
}
