//! The solid the rod of radius 0.5 about the x axis from x = −3 to 5 fills as it turns 120° about
//! the z axis.
use springrep::math::{Frame, Vec3};
use springrep::primitive::cylinder;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, turning, v3};

pub fn build() -> Result<Brep, String> {
    let end = Frame::new(v3(-3.0, 0.0, 0.0), Vec3::X, Vec3::Y).map_err(e)?;
    let rod = cylinder(&end, 0.5, 8.0, "rod").map_err(e)?;
    solid(&rod, &turning(0.0, deg(120.0)), "spin").map_err(e)
}
