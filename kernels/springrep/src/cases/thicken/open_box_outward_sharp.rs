//! Five faces of the cube from the origin to (10, 10, 10), an open tray, a sheet, thickened by 1 out
//! of the cube, the moved faces extended until they meet: the box from (-1, -1, -1) to (11, 11, 10)
//! less the cube.
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::thicken::{sheet_of, thickened_sheet};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the tray is five faces of the cube, all but its top (`end`), taken alone; their normals
    // point out of the cube
    let cube = block(&at([0., 0., 0.]), Vec3::new(10., 10., 10.), "cube").map_err(e)?;
    let tray = sheet_of(
        &cube,
        &[
            "cube.start",
            "cube.u_min",
            "cube.u_max",
            "cube.v_min",
            "cube.v_max",
        ],
    )?;
    thickened_sheet(&tray, 1., 0.)
}
