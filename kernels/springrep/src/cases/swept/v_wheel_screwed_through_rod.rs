//! The rod of radius 5 about the z axis from z = 0 to 20 less everything a V wheel (a sharp rim
//! of radius 10 about (14, 0, 0), leaning 43.30°) passes through as it is screwed about the z
//! axis, rising 30 a turn.
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::profile::Profile;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, polygon, revolved, rod, screwing, v3, within};

/// The V wheel's rim in (distance from its axis, along it): its tip 10 out, the V 2.5 deep with
/// flanks at 30° to the wheel's plane.
fn v_rim() -> Profile {
    let flank = 2.5 * deg(30.0).tan();
    polygon(&[(7.5, -flank), (10.0, 0.0), (7.5, flank)], "rim")
}

pub fn build() -> Result<Brep, String> {
    let h = (2.0 * PI * 4.5 / 30.0).atan();
    let along = v3(0.0, h.cos(), -h.sin());
    let wheel = revolved(
        &v_rim(),
        v3(14.0, 0.0, 0.0),
        along,
        v3(-1.0, 0.0, 0.0),
        "wheel",
    )?;
    // No sweep of the whole tool: a screw keeps each point's distance from the z axis, so only the
    // tool within 5.5 of it reaches the rod, and only that part is swept.
    let rim = within(&wheel, 5.5)?;
    let swept = solid(&rim, &screwing(30.0, deg(-144.0), deg(384.0)), "screw").map_err(e)?;
    difference(&rod()?, &swept).map_err(e)
}
