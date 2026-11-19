//! A creased sheet, the floor from (0, 0, 0) to (10, 8, 0) and the wall from (0, 0, 0) to (0, 8, 10),
//! thickened by 1 to the inside of its crease: the moved faces cross and are trimmed where they meet.
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::thicken::{sheet_of, thickened_sheet};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the floor, z = 0, and the wall, x = 0, are two faces of the block from the origin to
    // (10, 8, 10), taken alone; their normals, -z and -x, point to the outside of the crease
    let b = block(&at([0., 0., 0.]), Vec3::new(10., 8., 10.), "block").map_err(e)?;
    let angle = sheet_of(&b, &["block.start", "block.u_min"])?;
    // the inside of the crease is the sheet's back
    thickened_sheet(&angle, 0., 1.)
}
