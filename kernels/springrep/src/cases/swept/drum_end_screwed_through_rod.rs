//! The rod of radius 5 about the z axis from z = 0 to 20 less everything a cylinder of radius
//! 1.2 along the x axis from x = 4 to 12 passes through as it is screwed about the z axis,
//! rising 6 a turn.
use springrep::boolean::difference;
use springrep::math::{Frame, Vec3};
use springrep::primitive::cylinder;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, rod, screwing, v3};

pub fn build() -> Result<Brep, String> {
    let end = Frame::new(v3(4.0, 0.0, 0.0), Vec3::X, Vec3::Y).map_err(e)?;
    let mill = cylinder(&end, 1.2, 8.0, "mill").map_err(e)?;
    let swept = solid(&mill, &screwing(6.0, deg(-90.0), deg(1350.0)), "screw").map_err(e)?;
    difference(&rod()?, &swept).map_err(e)
}
