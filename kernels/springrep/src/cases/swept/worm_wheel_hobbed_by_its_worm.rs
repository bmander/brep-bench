//! A worm wheel's 30 teeth cut from the disc of radius 32 (z from −6 to 6) by its worm on the
//! line through (0, 40, 0) along x, turning in mesh.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep's sweep of a solid is exact only under a slide, a turn or a screw. The tooth (or thread) is carried by two turns at once, about the blank's axis and the mating one, so the space it cuts is bounded by envelopes of its flanks under that motion, which it cannot make exactly".into())
}
