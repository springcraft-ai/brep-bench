//! The solid a thick disc (radius 2 about the line through (2, 0, 0) along y, from y = 0 to 0.5)
//! fills as it turns 90° about the z axis.
use std::f64::consts::PI;

use springrep::math::{Frame, Vec3};
use springrep::primitive::cylinder;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{turning, v3};

pub fn build() -> Result<Brep, String> {
    let face = Frame::new(v3(2.0, 0.0, 0.0), Vec3::Y, Vec3::X).map_err(e)?;
    let disc = cylinder(&face, 2.0, 0.5, "disc").map_err(e)?;
    solid(&disc, &turning(0.0, PI / 2.0), "turn").map_err(e)
}
