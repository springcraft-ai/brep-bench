//! The rod of radius 5 about the z axis from z = 0 to 20 less everything a drafted bar of slot
//! section pointing at the axis passes through as it is screwed about the z axis, rising 6 a
//! turn.
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::loft::{Section, loft};
use springrep::math::{Frame, Vec3};
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{arc, deg, line, outline, rod, screwing, v3, within};

/// The slot in the plane x = `x`, drawn in (y, z): half discs of radius `r` about (±1.5, 0) and the
/// rectangle between them.
fn slot(x: f64, r: f64) -> Result<Section, String> {
    let half = PI / 2.0;
    let profile = outline(vec![
        line((-1.5, -r), (1.5, -r), "side"),
        arc((1.5, 0.0), r, -half, PI, "end"),
        line((1.5, r), (-1.5, r), "side"),
        arc((-1.5, 0.0), r, half, PI, "end"),
    ]);
    let frame = Frame::new(v3(x, 0.0, 0.0), Vec3::X, Vec3::Y).map_err(e)?;
    Ok(Section { profile, frame })
}

pub fn build() -> Result<Brep, String> {
    let draft = deg(3.0).tan();
    let bar = loft(
        &[slot(12.0, 1.0)?, slot(4.0, 1.0 - 8.0 * draft)?],
        true,
        "bar",
    )
    .map_err(e)?;
    // No sweep of the whole tool: a screw keeps each point's distance from the z axis, so only the
    // tool within 5.5 of it reaches the rod, and only that part is swept.
    let tip = within(&bar, 5.5)?;
    let swept = solid(&tip, &screwing(6.0, deg(-90.0), deg(1350.0)), "screw").map_err(e)?;
    difference(&rod()?, &swept).map_err(e)
}
