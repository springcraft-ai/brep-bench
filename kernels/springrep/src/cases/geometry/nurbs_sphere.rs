//! A ball of radius 5 bounded by one biquadratic rational B-spline face with a pole at each end.
use springrep::topo::Brep;

use crate::shared::nurbs::nurbs_ball;

pub fn build() -> Result<Brep, String> {
    nurbs_ball(0., "ball")
}
