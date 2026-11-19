//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a bull-nose
//! mill passes through as it turns about the z axis from −140° to 30°: a cylinder of radius 1 about
//! the upright line through (10, 0), from z = −0.5 to 0.5, its bottom corner rounded to 0.3.
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{arc, deg, line, outline, sector_block, turning, upright};

pub fn build() -> Result<Brep, String> {
    let meridian = outline(vec![
        line((0.0, -0.5), (0.7, -0.5), "bottom"),
        arc((0.7, -0.2), 0.3, -PI / 2.0, PI / 2.0, "round"),
        line((1.0, -0.2), (1.0, 0.5), "side"),
        line((1.0, 0.5), (0.0, 0.5), "top"),
        line((0.0, 0.5), (0.0, -0.5), "axis"),
    ]);
    let mill = upright(&meridian, 10.0, 0.0, "mill")?;
    let notch = solid(&mill, &turning(deg(-140.0), deg(30.0)), "notch").map_err(e)?;
    difference(&sector_block()?, &notch).map_err(e)
}
