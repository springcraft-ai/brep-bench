//! The rod of radius 5 about the z axis from z = 0 to z = 20, less everything the bar
//! 4 ≤ x ≤ 12, |y| ≤ 8, |z| ≤ 1 passes through as it is screwed about the z axis, rising 6 a turn,
//! from a turn of −120° to one of 600°, where it stops inside the rod.
use springrep::boolean::difference;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, deg, rod, screwing};

pub fn build() -> Result<Brep, String> {
    let bar = boxed([4.0, -8.0, -1.0], [12.0, 8.0, 1.0], "bar")?;
    let swept = solid(&bar, &screwing(6.0, deg(-120.0), deg(600.0)), "screw").map_err(e)?;
    difference(&rod()?, &swept).map_err(e)
}
