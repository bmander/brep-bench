//! The solid a torus (centre (3, 0, 2), axis along z, radii 1 and 0.5) fills as it turns by t/4
//! about the vertical line through (2, 0, 0) and then by −t about the line through (0, 0.5, 0.5)
//! along x, as t runs from −75° to 75°.
//!
//! Refused. cad-kernel has no sweep of a solid along a motion. Its only sweeps carry a planar
//! section or a ball: `build::prism` along a line, `build::revolve` about one, `build::canal` a
//! ball along a screw, a face family a section along a path. None of them is this solid: the two
//! turns about skew axes keep no line of the torus fixed and are no screw, so the boundary is the
//! envelope of the torus under the motion — at each instant the curve on it where its velocity
//! lies tangent to it — which cad-kernel does not compute.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel has no sweep of a solid along a motion: it sweeps only a planar section (along \
         a line, about an axis, along a path) or a ball (along a screw or a path), and the tumbling \
         torus's boundary is its envelope under two turns about skew axes, which it cannot build"
        .into())
}
