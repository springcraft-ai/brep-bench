//! The box (−10, −10, −5)–(10, 10, 0) less everything an end mill passes through as it turns 120°
//! about the z axis: the cylinder of radius 3 about the upright line through (1.5, 0), from z = −2
//! to 4, which the z axis passes through.
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::swept::{boxed, deg, turning};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let block = boxed([-10.0, -10.0, -5.0], [10.0, 10.0, 0.0], "block")?;
    let mill = cylinder(&at([1.5, 0.0, -2.0]), 3.0, 6.0, "mill").map_err(e)?;
    let pocket = solid(&mill, &turning(0.0, deg(120.0)), "pocket").map_err(e)?;
    difference(&block, &pocket).map_err(e)
}
