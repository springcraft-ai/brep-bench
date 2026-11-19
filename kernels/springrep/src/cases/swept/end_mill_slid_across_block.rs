//! The block (0, −5, 0)–(20, 5, 10) less everything an end mill passes through as it slides along
//! the x axis by 30: the cylinder of radius 2 about the upright line through (−5, 0), from z = 7
//! to 15.
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::swept::{boxed, v3};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let block = boxed([0.0, -5.0, 0.0], [20.0, 5.0, 10.0], "block")?;
    let mill = cylinder(&at([-5.0, 0.0, 7.0]), 2.0, 8.0, "mill").map_err(e)?;
    let slot = solid(&mill, &Motion::Slide(v3(30.0, 0.0, 0.0)), "slot").map_err(e)?;
    difference(&block, &slot).map_err(e)
}
