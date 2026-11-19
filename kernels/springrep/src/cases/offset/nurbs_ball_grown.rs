//! A ball of radius 5 bounded by one biquadratic rational B-spline face with a pole at each end,
//! grown by 2.
use springrep::edit::offset;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::nurbs::nurbs_ball;

pub fn build() -> Result<Brep, String> {
    offset(&nurbs_ball(0., "ball")?, 2., "grow").map_err(e)
}
