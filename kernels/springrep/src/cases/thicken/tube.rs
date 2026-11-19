//! The whole cylinder of radius 5 about the z axis from z = 0 to z = 10, a sheet, thickened by 1
//! away from the axis: the tube between radii 5 and 6.
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::thicken::{sheet_of, thickened_sheet};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the sheet is the side of a solid cylinder, taken alone, its normal away from the axis
    let can = cylinder(&at([0., 0., 0.]), 5., 10., "can").map_err(e)?;
    thickened_sheet(&sheet_of(&can, &["can.side"])?, 1., 0.)
}
