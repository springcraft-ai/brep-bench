//! The ball of radius 5 about the origin bored through along the z axis with a hole of radius 2,
//! the upper rim of the bore chamfered with two distances: 2√21 down the bore, its whole length,
//! and 1 on the ball, so the bore shrinks to its lower rim and the hole is a cone.
use springrep::boolean::difference;
use springrep::primitive::{cylinder, sphere};
use springrep::topo::Brep;

use crate::shared::chamfer::rim_by_two_distances;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ball = sphere(&at([0., 0., 0.]), 5., "ball").map_err(e)?;
    let bore = cylinder(&at([0., 0., -6.]), 2., 12., "bore").map_err(e)?;
    let body = difference(&ball, &bore).map_err(e)?;
    let top = 21f64.sqrt();
    rim_by_two_distances(&body, (2., top), "bore.side", 2. * top, 1.)
}
