//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything the drum of
//! radius 1 about the upright line through (10, 0), from z = −0.5 to 0.5, passes through as it
//! turns about the z axis from −140° to 30°.
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::swept::{deg, sector_block, turning};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let drum = cylinder(&at([10.0, 0.0, -0.5]), 1.0, 1.0, "drum").map_err(e)?;
    let notch = solid(&drum, &turning(deg(-140.0), deg(30.0)), "notch").map_err(e)?;
    difference(&sector_block()?, &notch).map_err(e)
}
