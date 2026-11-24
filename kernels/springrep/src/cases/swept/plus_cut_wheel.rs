//! A disc of radius 40 about the upright line through (−50, 0) cut by a plus-shaped prism
//! turning in mesh with it, as a gear shaper cuts.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep's sweep of a solid is exact only under a slide, a turn or a screw. Seen from the wheel the tool turns about one axis while being carried about another, so the wheel's teeth are bounded by envelopes of the plus's sides and the trochoids its corners trace, which it cannot make exactly".into())
}
