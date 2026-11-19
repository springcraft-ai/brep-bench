//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a frustum
//! about the upright line through (10, 0) (radius 0.4 at z = −0.6 to 1 at z = 1.4) passes
//! through as it is screwed about the z axis, rising 2 a turn, from −140° to 140°.
use springrep::boolean::difference;
use springrep::primitive::cone;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, screwing, sector_block};

pub fn build() -> Result<Brep, String> {
    let foot = crate::shared::at([10.0, 0.0, -0.6]);
    let cutter = cone(&foot, 0.4, 1.0, 2.0, "cutter").map_err(e)?;
    let swept = solid(&cutter, &screwing(2.0, deg(-140.0), deg(140.0)), "screw").map_err(e)?;
    difference(&sector_block()?, &swept).map_err(e)
}
