//! The block (0, −10, −10)–(30, 10, 0) less everything a V bit passes through as it slides along
//! the x axis by 15: the cone about the upright line through (5, 0), pointed at (5, 0, −2), opening
//! at 90° to a flat top of radius 5 at z = 3.
use springrep::boolean::difference;
use springrep::primitive::cone;
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::swept::{boxed, v3};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let block = boxed([0.0, -10.0, -10.0], [30.0, 10.0, 0.0], "block")?;
    let bit = cone(&at([5.0, 0.0, -2.0]), 0.0, 5.0, 5.0, "bit").map_err(e)?;
    let groove = solid(&bit, &Motion::Slide(v3(15.0, 0.0, 0.0)), "groove").map_err(e)?;
    difference(&block, &groove).map_err(e)
}
