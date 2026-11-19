//! The rod of radius 5 about the z axis from z = 0 to 20 less everything a cone pointed at (4,
//! 0, 0) along the x axis, opening at 60°, passes through as it is screwed about the z axis,
//! rising 6 a turn.
use springrep::boolean::difference;
use springrep::math::{Frame, Vec3};
use springrep::primitive::cone;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, rod, screwing, v3, within};

pub fn build() -> Result<Brep, String> {
    let point = Frame::new(v3(4.0, 0.0, 0.0), Vec3::X, Vec3::Y).map_err(e)?;
    let bit = cone(&point, 0.0, 8.0 * deg(30.0).tan(), 8.0, "bit").map_err(e)?;
    // No sweep of the whole tool: a screw keeps each point's distance from the z axis, so only the
    // tool within 5.5 of it reaches the rod, and only that part is swept.
    let tip = within(&bit, 5.5)?;
    let swept = solid(&tip, &screwing(6.0, deg(-90.0), deg(1350.0)), "screw").map_err(e)?;
    difference(&rod()?, &swept).map_err(e)
}
