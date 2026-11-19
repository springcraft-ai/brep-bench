//! The sphere of radius 5 about the origin, a closed sheet, thickened by 1 outward: the hollow ball
//! between radii 5 and 6.
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::thicken::{sheet_of, thickened_sheet};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the sheet is the skin of a ball, taken alone, its normal outward
    let ball = sphere(&at([0., 0., 0.]), 5., "ball").map_err(e)?;
    thickened_sheet(&sheet_of(&ball, &["ball.skin"])?, 1., 0.)
}
