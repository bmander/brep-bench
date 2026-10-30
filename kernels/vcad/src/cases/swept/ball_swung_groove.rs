//! A block, the box from (−6, −6, −8) to (18, 18, 0), with a groove cut into its top face by a
//! ball of radius 3 whose centre swings a quarter turn about the z axis in the top face, from
//! (12, 0, 0) to (0, 12, 0).
//!
//! vcad has no sweep of a solid, so the ball's sweep is built from what it is by elementary
//! geometry: every point within 3 of the quarter circle its centre runs along. That is the tube the
//! ball's great disc square to the circle fills as it turns, the quarter of the torus about the
//! z axis (radii 12 and 3) between the half planes x = 0 and y = 0, with a ball at each end: half
//! of each lies inside the tube.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const BLOCK_MIN: [f64; 3] = [-6.0, -6.0, -8.0];
const BLOCK_MAX: [f64; 3] = [18.0, 18.0, 0.0];
const BALL_RADIUS: f64 = 3.0;
/// The ball's centre's distance from the z axis.
const SWING: f64 = 12.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let [x0, y0, z0] = BLOCK_MIN;
    let [x1, y1, z1] = BLOCK_MAX;
    let block = Solid::cube(x1 - x0, y1 - y0, z1 - z0).translate(x0, y0, z0);
    // vcad's torus is centred on the origin about the z axis; the quarter of it the ball turns
    // through is what lies in a box on the quadrant x, y ≥ 0, reaching past the torus elsewhere
    let torus = Solid::torus(SWING, BALL_RADIUS, SEGMENTS);
    let reach = 2.0 * (SWING + BALL_RADIUS);
    let quadrant = Solid::cube(reach, reach, reach).translate(0.0, 0.0, -reach / 2.0);
    let tube = torus.try_intersection(&quadrant)?;
    let ball = |x: f64, y: f64| Solid::sphere(BALL_RADIUS, SEGMENTS).translate(x, y, 0.0);
    // the tube and the ball at each end, each cut in turn
    let grooved = block.try_difference(&tube)?;
    let grooved = grooved.try_difference(&ball(SWING, 0.0))?;
    Ok(grooved.try_difference(&ball(0.0, SWING))?)
}
