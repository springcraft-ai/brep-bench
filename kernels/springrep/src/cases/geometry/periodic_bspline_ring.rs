//! A ring about the z axis whose section, in the xz plane, is the closed uniform quadratic B-spline
//! on the square of corners (13, 3), (7, 3), (7, −3), (13, −3), turned a whole turn.
use springrep::geom::BSpline;
use springrep::math::Vec3;
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::revolve;
use springrep::topo::Brep;
use std::f64::consts::TAU;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // sketch coordinates on the xz plane: (x, z)
    let corners = [[13., 3.], [7., 3.], [7., -3.], [13., -3.]];
    let polygon: Vec<Vec3> = corners.iter().map(|c| Vec3::new(c[0], c[1], 0.)).collect();
    let curve = BSpline::periodic_uniform(2, &polygon).map_err(e)?;
    let section = Segment::Spline {
        curve,
        reversed: false,
    };
    let section = Profile::outline(vec![NamedSegment::new(section, "section")]);
    let xz = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    revolve(&section, &xz, Vec3::ZERO, Vec3::Z, TAU, "ring").map_err(e)
}
