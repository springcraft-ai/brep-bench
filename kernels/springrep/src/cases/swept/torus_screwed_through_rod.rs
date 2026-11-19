//! The rod of radius 5 about the z axis from z = 0 to 20 less everything a torus wheel (radii 6
//! and 2 about (11.5, 0, 0), leaning 49.04°) passes through as it is screwed about the z axis,
//! rising 30 a turn.
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::math::{Frame, Vec3};
use springrep::primitive::torus;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, rod, screwing, v3, within};

pub fn build() -> Result<Brep, String> {
    let h = (2.0 * PI * 5.5 / 30.0).atan();
    let axis = Frame::new(v3(11.5, 0.0, 0.0), v3(0.0, h.cos(), -h.sin()), Vec3::X).map_err(e)?;
    let wheel = torus(&axis, 6.0, 2.0, "wheel").map_err(e)?;
    // No sweep of the whole tool: a screw keeps each point's distance from the z axis, so only the
    // tool within 5.5 of it reaches the rod, and only that part is swept.
    let rim = within(&wheel, 5.5)?;
    let swept = solid(&rim, &screwing(30.0, deg(-108.0), deg(348.0)), "screw").map_err(e)?;
    difference(&rod()?, &swept).map_err(e)
}
