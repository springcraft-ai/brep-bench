//! The solid a square bar fills as it turns a quarter turn about the z axis: the prism from z = 0
//! to 3 on the square with corners (4, 0), (6, 2), (8, 0) and (6, −2), stood on a corner.
use std::f64::consts::FRAC_PI_2;

use springrep::sweep::{extrude, solid};
use springrep::topo::Brep;

use crate::shared::swept::{polygon, turning};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let square = polygon(&[(4.0, 0.0), (6.0, -2.0), (8.0, 0.0), (6.0, 2.0)], "side");
    let bar = extrude(&square, &at([0.0, 0.0, 0.0]), 3.0, "bar").map_err(e)?;
    solid(&bar, &turning(0.0, FRAC_PI_2), "swept").map_err(e)
}
