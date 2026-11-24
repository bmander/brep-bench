//! A torus swept continuously under two turns. Only public kernel operations build geometry.
use cad_kernel::build::torus;
use cad_kernel::swept::closed::ring;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

/// Case dimensions and angles (degrees). The torus's major radius is one and its axis is z.
pub struct Tumble {
    pub centre: [f64; 3],
    pub hub: f64,
    pub tube: f64,
    pub ratio: f64,
    pub lean: f64,
    pub heading: f64,
    pub height: f64,
    pub observer: f64,
    pub roll: [f64; 2],
}

impl Tumble {
    /// The tolerance the sweep is built to, its envelope fitted within it: a fifty-thousandth of
    /// the tube's radius. A Boolean of two such sweeps is asked at it, as no finer than what it
    /// combines.
    pub fn tolerance(&self) -> f64 {
        self.tube / 50_000.
    }

    pub fn build(self) -> Result<Brep, String> {
        let tool = torus(self.centre, [0., 0., 1.], 1., self.tube)?;
        let (sl, cl) = self.lean.to_radians().sin_cos();
        let (sa, ca) = self.heading.to_radians().sin_cos();
        let own = Trajectory::Twist(Screw {
            origin: [self.hub, 0., 0.], axis: [0., 0., 1.], ratio: self.ratio, advance: 0.,
        });
        let frame = Trajectory::Twist(Screw {
            origin: [0., self.height * cl, self.height * sl],
            axis: [ca, sa * cl, sa * sl], ratio: self.observer, advance: 0.,
        });
        let path = Trajectory::Relative { source: Box::new(own), observer: Box::new(frame) };
        // The existing tumbling-torus case's bars, scaled with tube radius: membership reads
        // within r/250, envelope fitting within r/50000. A thin tool must not inherit the
        // original thick tool's absolute mesh sag. No sampled-pose union or fitted mesh here.
        ring(&tool, &path, self.roll.map(f64::to_radians), self.tube / 250., self.tolerance())
            .map_err(|why| format!("the torus's continuous sweep not built: {why:?}"))
    }
}
