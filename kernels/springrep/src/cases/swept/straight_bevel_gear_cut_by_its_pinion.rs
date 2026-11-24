//! A bevel gear blank with one tooth space cut by a tooth of its pinion turning in mesh, the
//! shafts square and meeting.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep's sweep of a solid is exact only under a slide, a turn or a screw. The tooth (or thread) is carried by two turns at once, about the blank's axis and the mating one, so the space it cuts is bounded by envelopes of its flanks under that motion, which it cannot make exactly".into())
}
