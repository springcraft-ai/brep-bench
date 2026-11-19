//! The upper half of the sphere of radius 5 about the origin, a sheet, thickened by 1 to each side:
//! the dome between radii 4 and 6.
use std::f64::consts::FRAC_PI_2;

use springrep::topo::Brep;

use crate::shared::thicken::{arc, line, sheet_of, thickened_sheet, turned_whole};

pub fn build() -> Result<Brep, String> {
    // the sheet is the dome of a half ball, taken alone
    let half = turned_whole(
        vec![
            line([0., 0.], [5., 0.], "flat"),
            arc([0., 0.], 5., 0., FRAC_PI_2, "dome"),
            line([0., 5.], [0., 0.], "axis"),
        ],
        "half",
    )?;
    thickened_sheet(&sheet_of(&half, &["half.dome"])?, 1., 1.)
}
