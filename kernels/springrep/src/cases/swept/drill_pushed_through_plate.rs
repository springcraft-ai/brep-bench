//! The plate (0, −5, 0)–(20, 5, 4) less everything a drill passes through as it slides straight
//! down by 20: a cylinder of radius 2 about the upright line through (10, 0) up to z = 14, ended
//! below in a 118° cone pointed at (10, 0, 6).
use springrep::boolean::difference;
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, deg, polygon, upright, v3};

pub fn build() -> Result<Brep, String> {
    let plate = boxed([0.0, -5.0, 0.0], [20.0, 5.0, 4.0], "plate")?;
    let rim = 6.0 + 2.0 / deg(59.0).tan();
    let meridian = polygon(&[(0.0, 6.0), (2.0, rim), (2.0, 14.0), (0.0, 14.0)], "drill");
    let drill = upright(&meridian, 10.0, 0.0, "drill")?;
    let hole = solid(&drill, &Motion::Slide(v3(0.0, 0.0, -20.0)), "hole").map_err(e)?;
    difference(&plate, &hole).map_err(e)
}
