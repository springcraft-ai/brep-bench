//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a pin passes
//! through as it turns about the z axis from −140° to 140°: the cylinder of radius 0.5 about the x
//! axis from x = 6 to 11.
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::swept::{deg, sector_block, turning};
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let axis = frame([6.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])?;
    let pin = cylinder(&axis, 0.5, 5.0, "pin").map_err(e)?;
    let slot = solid(&pin, &turning(deg(-140.0), deg(140.0)), "slot").map_err(e)?;
    difference(&sector_block()?, &slot).map_err(e)
}
