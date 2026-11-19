//! A prism from z = 0 to z = 10 on the closed uniform quadratic B-spline whose control polygon is
//! the square of corners (10, 10), (−10, 10), (−10, −10), (10, −10): four parabolic arcs from the
//! middle of each side to the next.
use springrep::geom::BSpline;
use springrep::math::Vec3;
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let corners = [[10., 10.], [-10., 10.], [-10., -10.], [10., -10.]];
    let polygon: Vec<Vec3> = corners.iter().map(|c| Vec3::new(c[0], c[1], 0.)).collect();
    let curve = BSpline::periodic_uniform(2, &polygon).map_err(e)?;
    let side = Segment::Spline {
        curve,
        reversed: false,
    };
    let section = Profile::outline(vec![NamedSegment::new(side, "side")]);
    extrude(&section, &at([0., 0., 0.]), 10., "prism").map_err(e)
}
