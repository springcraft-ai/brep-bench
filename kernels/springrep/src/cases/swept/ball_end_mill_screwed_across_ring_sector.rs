//! The ring sector (radii 8 to 12, heights −2 to 2, angles −30° to 30°) less everything a ball-
//! end mill (radius 1 about the upright line through (10, 0), its ball centered at (10, 0, 0))
//! passes through as it is screwed about the z axis, rising 3 a turn, from −60° to 60°.
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{arc, deg, line, outline, ring_sector, screwing, upright};

pub fn build() -> Result<Brep, String> {
    let block = ring_sector(8.0, 12.0, -2.0, 2.0, deg(-30.0), deg(30.0))?;
    // the ball's quarter arc from the tip out to the side, the side up to z = 3, the flat top
    let meridian = outline(vec![
        arc((0.0, 0.0), 1.0, -PI / 2.0, PI / 2.0, "ball"),
        line((1.0, 0.0), (1.0, 3.0), "side"),
        line((1.0, 3.0), (0.0, 3.0), "top"),
        line((0.0, 3.0), (0.0, -1.0), "axis"),
    ]);
    let mill = upright(&meridian, 10.0, 0.0, "mill")?;
    let swept = solid(&mill, &screwing(3.0, deg(-60.0), deg(60.0)), "screw").map_err(e)?;
    difference(&block, &swept).map_err(e)
}
