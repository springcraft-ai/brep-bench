//! A sheet bent as sheet metal, a floor, a quarter-cylinder bend of radius 2 and an upright wall,
//! swept 8 along y, thickened by 3 to the inside of the bend, more than the bend's radius: the bend
//! moved 3 turns inside out and drops, and the floor and wall moved meet in a concave edge.
use springrep::topo::Brep;

use crate::shared::thicken::{flange, sheet_of, thickened_sheet};

pub fn build() -> Result<Brep, String> {
    // the sheet's faces face away from the bend's axis: the inside of the bend is its back
    let sheet = sheet_of(&flange()?, &["flange.floor", "flange.bend", "flange.wall"])?;
    thickened_sheet(&sheet, 0., 3.)
}
