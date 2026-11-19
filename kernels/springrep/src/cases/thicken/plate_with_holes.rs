//! The square from (-10, -10, 0) to (10, 10, 0) with two holes of radius 3 about (-5, 0, 0) and
//! (5, 0, 0), a sheet, thickened by 1 to each side: a plate from z = -1 to 1 holed right through.
use springrep::boolean::difference_all;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::thicken::{sheet_of, thickened_sheet};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the sheet is the top of a holed plate under it, taken alone
    let plate = block(&at([-10., -10., -2.]), Vec3::new(20., 20., 2.), "plate").map_err(e)?;
    let left = cylinder(&at([-5., 0., -3.]), 3., 4., "left").map_err(e)?;
    let right = cylinder(&at([5., 0., -3.]), 3., 4., "right").map_err(e)?;
    let holed = difference_all(&plate, &[&left, &right]).map_err(e)?;
    thickened_sheet(&sheet_of(&holed, &["plate.end"])?, 1., 1.)
}
