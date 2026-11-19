//! The rod of radius 5 about the z axis from z = 0 to z = 20 less everything the box
//! 2.5 ≤ x ≤ 3.5, |y| ≤ 0.5, 2 ≤ z ≤ 3 passes through as it is screwed about the z axis, rising 4
//! a turn, from a turn of 0° to one of 720°: a sealed helical tunnel.
use springrep::boolean::difference;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, deg, rod, screwing};

pub fn build() -> Result<Brep, String> {
    let bar = boxed([2.5, -0.5, 2.0], [3.5, 0.5, 3.0], "bar")?;
    let swept = solid(&bar, &screwing(4.0, 0.0, deg(720.0)), "screw").map_err(e)?;
    difference(&rod()?, &swept).map_err(e)
}
