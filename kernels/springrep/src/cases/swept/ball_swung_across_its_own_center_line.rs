//! The solid a ball of radius 2 centered at (1, 0, 0) fills as it turns 120° about the z axis,
//! which passes through it.
use springrep::primitive::sphere;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::swept::{deg, turning};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ball = sphere(&at([1.0, 0.0, 0.0]), 2.0, "ball").map_err(e)?;
    solid(&ball, &turning(0.0, deg(120.0)), "swept").map_err(e)
}
