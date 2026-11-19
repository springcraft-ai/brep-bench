//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a drum
//! passes through as it is screwed about the z axis, rising 0.5 a turn, from a turn of −140° to one
//! of 140°: the cylinder of radius 1 about the upright line through (10, 0), from z = −0.5 to 0.5.
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, screwing, sector_block};

pub fn build() -> Result<Brep, String> {
    let drum = cylinder(&crate::shared::at([10.0, 0.0, -0.5]), 1.0, 1.0, "drum").map_err(e)?;
    let swept = solid(&drum, &screwing(0.5, deg(-140.0), deg(140.0)), "screw").map_err(e)?;
    difference(&sector_block()?, &swept).map_err(e)
}
