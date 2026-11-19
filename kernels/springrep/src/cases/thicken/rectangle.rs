//! The rectangle in the xy plane from (0, 0, 0) to (10, 8, 0), a sheet, thickened by 2 to its +z
//! side: the box from the origin to (10, 8, 2).
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::thicken::{sheet_of, thickened_sheet};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the sheet is the top of a block under it, taken alone, its normal +z
    let b = block(&at([0., 0., -1.]), Vec3::new(10., 8., 1.), "plate").map_err(e)?;
    thickened_sheet(&sheet_of(&b, &["plate.end"])?, 2., 0.)
}
