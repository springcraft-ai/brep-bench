//! The plate (0, −5, 0)–(20, 5, 4) less everything a drum passes through as it slides by 8 along
//! (0.3, 0, √0.91): the cylinder of radius 2 about the line y = 0, z = −5, from x = 6 to 14.
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::swept::{boxed, v3};
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let plate = boxed([0.0, -5.0, 0.0], [20.0, 5.0, 4.0], "plate")?;
    let axis = frame([6.0, 0.0, -5.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])?;
    let drum = cylinder(&axis, 2.0, 8.0, "drum").map_err(e)?;
    let v = v3(0.3, 0.0, 0.91f64.sqrt()) * 8.0;
    let pocket = solid(&drum, &Motion::Slide(v), "pocket").map_err(e)?;
    difference(&plate, &pocket).map_err(e)
}
