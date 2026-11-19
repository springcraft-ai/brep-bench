//! The plate (0, −5, 0)–(20, 5, 4) less everything a drum passes through as it slides straight up
//! by 16: the cylinder of radius 2 about the line y = 0, z = −5, from x = 6 to 14.
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::swept::{boxed, v3};
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let plate = boxed([0.0, -5.0, 0.0], [20.0, 5.0, 4.0], "plate")?;
    let end = frame([6.0, 0.0, -5.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])?;
    let drum = cylinder(&end, 2.0, 8.0, "drum").map_err(e)?;
    let slot = solid(&drum, &Motion::Slide(v3(0.0, 0.0, 16.0)), "slot").map_err(e)?;
    difference(&plate, &slot).map_err(e)
}
