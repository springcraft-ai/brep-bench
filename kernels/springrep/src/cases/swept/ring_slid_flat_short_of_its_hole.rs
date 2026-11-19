//! The solid a ring fills as it slides 6 along x: the torus about the z axis, its tube's centers
//! 6 from it and its tube of radius 2.
use springrep::math::Frame;
use springrep::primitive::torus;
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::v3;

pub fn build() -> Result<Brep, String> {
    let ring = torus(&Frame::WORLD, 6.0, 2.0, "ring").map_err(e)?;
    solid(&ring, &Motion::Slide(v3(6.0, 0.0, 0.0)), "slide").map_err(e)
}
