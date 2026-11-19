//! The plate (−15, −15, 0)–(15, 15, 4) less everything an ellipsoid passes through as it slides
//! along (4, 3, 12): semi-axes 3, 2 and 1.5 along x, y and z about (−2, −1.5, −4).
use springrep::boolean::difference;
use springrep::primitive::ellipsoid;
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::swept::{boxed, v3};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = boxed([-15.0, -15.0, 0.0], [15.0, 15.0, 4.0], "plate")?;
    let ball = ellipsoid(&at([-2.0, -1.5, -4.0]), v3(3.0, 2.0, 1.5), "ball").map_err(e)?;
    let hole = solid(&ball, &Motion::Slide(v3(4.0, 3.0, 12.0)), "hole").map_err(e)?;
    difference(&plate, &hole).map_err(e)
}
