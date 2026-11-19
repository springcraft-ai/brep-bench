//! The rod of radius 5 about the z axis from z = 0 to z = 20, less everything the bar
//! 4.5 ≤ x ≤ 12, |y| ≤ 8, |z| ≤ 0.45 passes through as it is screwed about the z axis, rising 1 a
//! turn, from a turn of −180° to one of 7380°: a thread whose crests are sharp.
use springrep::boolean::difference;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, deg, rod, screwing, within};

pub fn build() -> Result<Brep, String> {
    let bar = boxed([4.5, -8.0, -0.45], [12.0, 8.0, 0.45], "bar")?;
    // No sweep of the whole tool: a screw keeps each point's distance from the z axis, so only the
    // tool within 5.5 of it reaches the rod, and only that part is swept.
    let tip = within(&bar, 5.5)?;
    let swept = solid(&tip, &screwing(1.0, deg(-180.0), deg(7380.0)), "screw").map_err(e)?;
    difference(&rod()?, &swept).map_err(e)
}
