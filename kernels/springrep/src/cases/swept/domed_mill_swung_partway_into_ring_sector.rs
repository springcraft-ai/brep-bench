//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a domed mill
//! passes through as it turns about the z axis from −140° to 30°: a cylinder of radius 1 about the
//! upright line through (10, 0) up to z = 3, ended below by a cap of the sphere of radius 2 about
//! (10, 0, 1.2).
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{arc, deg, line, outline, sector_block, turning, upright};

pub fn build() -> Result<Brep, String> {
    let rim = 1.2 - 3f64.sqrt();
    // the meridian: the cap's arc from its lowest point up to the sharp rim, the side, the top
    let meridian = outline(vec![
        arc((0.0, 1.2), 2.0, -PI / 2.0, PI / 6.0, "cap"),
        line((1.0, rim), (1.0, 3.0), "side"),
        line((1.0, 3.0), (0.0, 3.0), "top"),
        line((0.0, 3.0), (0.0, -0.8), "axis"),
    ]);
    let mill = upright(&meridian, 10.0, 0.0, "mill")?;
    let notch = solid(&mill, &turning(deg(-140.0), deg(30.0)), "notch").map_err(e)?;
    difference(&sector_block()?, &notch).map_err(e)
}
