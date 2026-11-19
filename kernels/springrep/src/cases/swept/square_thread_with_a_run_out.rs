//! The rod of radius 5 about the z axis from z = 0 to z = 20, less everything the bar
//! 4 ≤ x ≤ 12, |y| ≤ 8, |z| ≤ 1 passes through as it is screwed about the z axis, rising 6 a turn,
//! from a turn of −90° to one of 900° (pointing along −x, its tip at (−4, 0, 15)), then slid 3
//! along −x, out of its groove.
use springrep::boolean::difference;
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, deg, rod, screwing, unite, v3};

pub fn build() -> Result<Brep, String> {
    let bar = boxed([4.0, -8.0, -1.0], [12.0, 8.0, 1.0], "bar")?;
    let thread = solid(&bar, &screwing(6.0, deg(-90.0), deg(900.0)), "screw").map_err(e)?;
    // The path is two pieces, each swept on its own: at a turn of 900°, where the second starts,
    // the bar is turned half a turn and raised 15, spanning −12 ≤ x ≤ −4.
    let end = boxed([-12.0, -8.0, 14.0], [-4.0, 8.0, 16.0], "bar")?;
    let run_out = solid(&end, &Motion::Slide(v3(-3.0, 0.0, 0.0)), "slide").map_err(e)?;
    difference(&rod()?, &unite(&[&thread, &run_out])?).map_err(e)
}
