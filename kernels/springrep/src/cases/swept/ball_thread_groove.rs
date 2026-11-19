//! The rod of radius 5 about the z axis from z = 0 to 20 less everything a ball of radius 1
//! passes through as its center runs six turns of the helix of radius 5 rising 4 a turn, from
//! (5, 0, −2).
use springrep::boolean::difference;
use springrep::primitive::sphere;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, rod, screwing};

pub fn build() -> Result<Brep, String> {
    let ball = sphere(&crate::shared::at([5.0, 0.0, -2.0]), 1.0, "ball").map_err(e)?;
    let swept = solid(&ball, &screwing(4.0, 0.0, deg(2160.0)), "screw").map_err(e)?;
    difference(&rod()?, &swept).map_err(e)
}
