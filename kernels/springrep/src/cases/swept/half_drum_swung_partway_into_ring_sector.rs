//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything half a drum
//! passes through as it turns about the z axis from −140° to 30°: the half with y ≤ 0 of the drum
//! of radius 1 about the upright line through (10, 0), from z = −0.5 to 0.5, flat face first.
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::sweep::{extrude, solid};
use springrep::topo::Brep;

use crate::shared::swept::{arc, deg, line, outline, sector_block, turning};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let half = outline(vec![
        arc((10.0, 0.0), 1.0, PI, PI, "side"),
        line((11.0, 0.0), (9.0, 0.0), "face"),
    ]);
    let tool = extrude(&half, &at([0.0, 0.0, -0.5]), 1.0, "half_drum").map_err(e)?;
    let notch = solid(&tool, &turning(deg(-140.0), deg(30.0)), "notch").map_err(e)?;
    difference(&sector_block()?, &notch).map_err(e)
}
