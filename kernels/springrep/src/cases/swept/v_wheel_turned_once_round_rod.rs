//! The rod of radius 5 about the z axis from z = 0 to 20 less everything a V wheel (a sharp rim
//! of radius 10 about the line through (14, 0, 10) leaning 20° across the turn) passes through
//! as it turns once about the z axis.
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::profile::Profile;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, polygon, revolved, rod, turning, v3};

/// The V wheel's rim in (distance from its axis, along it): its tip 10 out, the V 2.5 deep with
/// flanks at 30° to the wheel's plane.
fn v_rim() -> Profile {
    let flank = 2.5 * deg(30.0).tan();
    polygon(&[(7.5, -flank), (10.0, 0.0), (7.5, flank)], "rim")
}

pub fn build() -> Result<Brep, String> {
    let (s, c) = deg(20.0).sin_cos();
    let wheel = revolved(
        &v_rim(),
        v3(14.0, 0.0, 10.0),
        v3(0.0, s, c),
        v3(-1.0, 0.0, 0.0),
        "wheel",
    )?;
    let swept = solid(&wheel, &turning(0.0, 2.0 * PI), "turn").map_err(e)?;
    difference(&rod()?, &swept).map_err(e)
}
