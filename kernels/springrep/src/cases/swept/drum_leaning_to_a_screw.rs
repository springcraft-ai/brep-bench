//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a drum
//! leaning 25° toward +y about (10, 0, 0) passes through as it is screwed about the z axis,
//! rising 1 a turn, from −140° to 140°.
use springrep::boolean::difference;
use springrep::math::{Frame, Vec3};
use springrep::primitive::cylinder;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, screwing, sector_block, v3};

pub fn build() -> Result<Brep, String> {
    let lean = deg(25.0);
    let axis = v3(0.0, lean.sin(), lean.cos());
    let end = Frame::new(v3(10.0, 0.0, 0.0) - axis * 1.5, axis, Vec3::X).map_err(e)?;
    let drum = cylinder(&end, 0.8, 3.0, "drum").map_err(e)?;
    let swept = solid(&drum, &screwing(1.0, deg(-140.0), deg(140.0)), "screw").map_err(e)?;
    difference(&sector_block()?, &swept).map_err(e)
}
