//! The ring between radii 3 and 8 about the z axis, heights −2.5 to 2.5, less everything a drum
//! (radius 1, length 3, leaning 25° in the plane y = 1.2 about (5, 1.2, 0)) passes through as it
//! turns 100° about the z axis.
use springrep::boolean::difference;
use springrep::math::{Frame, Vec3};
use springrep::primitive::cylinder;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, turning, v3};

pub fn build() -> Result<Brep, String> {
    let floor = crate::shared::at([0.0, 0.0, -2.5]);
    let ring = difference(
        &cylinder(&floor, 8.0, 5.0, "ring").map_err(e)?,
        &cylinder(&floor, 3.0, 5.0, "bore").map_err(e)?,
    )
    .map_err(e)?;
    let lean = deg(25.0);
    let axis = v3(lean.sin(), 0.0, lean.cos());
    let end = Frame::new(v3(5.0, 1.2, 0.0) - axis * 1.5, axis, Vec3::X).map_err(e)?;
    let drum = cylinder(&end, 1.0, 3.0, "drum").map_err(e)?;
    let swept = solid(&drum, &turning(0.0, deg(100.0)), "turn").map_err(e)?;
    difference(&ring, &swept).map_err(e)
}
