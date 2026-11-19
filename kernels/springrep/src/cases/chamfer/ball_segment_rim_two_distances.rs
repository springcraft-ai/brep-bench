//! The ball of radius 5 about the origin cut flat at z = 3, the rim of its flat face chamfered
//! with two distances, 2 across the flat face and 1 on the ball.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, sphere};
use springrep::topo::Brep;

use crate::shared::chamfer::rim_by_two_distances;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ball = sphere(&at([0., 0., 0.]), 5., "ball").map_err(e)?;
    // everything above z = 3 goes; the block's floor becomes the flat face
    let above = block(&at([-10., -10., 3.]), Vec3::new(20., 20., 10.), "cut").map_err(e)?;
    let body = difference(&ball, &above).map_err(e)?;
    rim_by_two_distances(&body, (4., 3.), "cut.start", 2., 1.)
}
