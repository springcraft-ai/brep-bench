//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a drill
//! passes through as it turns about the z axis from −140° to 30°: the drill is a cylinder of radius
//! 1 about the upright line through (10, 0) up to z = 3, ended below in a 118° cone pointed at
//! (10, 0, −0.8).
use springrep::boolean::difference;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, polygon, sector_block, turning, upright};

pub fn build() -> Result<Brep, String> {
    let rim = -0.8 + 1.0 / deg(59.0).tan();
    // the drill's meridian, out from its axis
    let meridian = polygon(&[(0.0, -0.8), (1.0, rim), (1.0, 3.0), (0.0, 3.0)], "drill");
    let drill = upright(&meridian, 10.0, 0.0, "drill")?;
    let notch = solid(&drill, &turning(deg(-140.0), deg(30.0)), "notch").map_err(e)?;
    difference(&sector_block()?, &notch).map_err(e)
}
