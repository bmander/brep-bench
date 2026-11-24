//! A cylinder blank of radius 50 and height 45 machined into a dome of radius 40 by a five-axis
//! program of four cutters: roughing rings, 180 meridian and 15 polar finishing passes with a
//! ball-end mill held square to the dome, twelve spot faces and twelve drilled holes.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep's sweep of a solid takes one motion, not a chain of moves: the finishing passes turn a leaning ball-end mill \
         about level lines and the z axis, over a thousand moves, each of whose sweeps would have \
         to be pieced together from tori, balls and the shank's revolved silhouettes; the ball's \
         joins between pieces are the tangent \
         ball-and-cylinder contacts that already fail in cnc/ball-end-surface-raster \
         (\"numerical failure: face split: a hole lies in no region\")"
        .into())
}
