//! A cylinder of radius 10 along the x axis with a square pad on top, the square |x|, |y| ≤ 3 at
//! z = 15 extruded down until it meets the cylinder's side, and joined to it.
use springrep::boolean::union;
use springrep::math::Vec2;
use springrep::primitive::cylinder;
use springrep::profile::Profile;
use springrep::sweep::{Meet, extrude_to_next};
use springrep::topo::Brep;

use crate::shared::{at, e, frame};

pub fn build() -> Result<Brep, String> {
    let along_x = frame([-20., 0., 0.], [1., 0., 0.], [0., 1., 0.])?;
    let body = cylinder(&along_x, 10., 40., "cyl").map_err(e)?;
    let square = Profile::rect(Vec2::ZERO, 6., 6.);
    // down (against the sketch plane's normal +z) into the cylinder
    let pad =
        extrude_to_next(&square, &at([0., 0., 15.]), true, &body, Meet::Into, "pad").map_err(e)?;
    union(&body, &pad).map_err(e)
}
