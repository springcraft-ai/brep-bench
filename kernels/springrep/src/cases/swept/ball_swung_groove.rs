//! The box (−6, −6, −8)–(18, 18, 0) with a groove cut by a ball of radius 3 whose center swings a
//! quarter turn about the z axis from (12, 0, 0) to (0, 12, 0).
use std::f64::consts::FRAC_PI_2;

use springrep::boolean::difference;
use springrep::primitive::sphere;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::swept::{boxed, turning};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let block = boxed([-6.0, -6.0, -8.0], [18.0, 18.0, 0.0], "block")?;
    let ball = sphere(&at([12.0, 0.0, 0.0]), 3.0, "ball").map_err(e)?;
    let groove = solid(&ball, &turning(0.0, FRAC_PI_2), "groove").map_err(e)?;
    difference(&block, &groove).map_err(e)
}
