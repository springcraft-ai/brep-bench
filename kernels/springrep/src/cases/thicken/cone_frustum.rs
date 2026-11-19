//! The side of the cone frustum about the z axis whose radius runs from 6 at z = 0 to 3 at z = 4, a
//! sheet, thickened by 1 along its normal away from the axis: a band whose rims sweep cones.
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::thicken::{sheet_of, thickened_sheet};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the sheet is the side of the solid frustum, taken alone, its normal away from the axis
    let frustum = cone(&at([0., 0., 0.]), 6., 3., 4., "frustum").map_err(e)?;
    thickened_sheet(&sheet_of(&frustum, &["frustum.side"])?, 1., 0.)
}
