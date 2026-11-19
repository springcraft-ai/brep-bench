//! The cap of the sphere of radius 10 about the origin where z >= 6, a sheet, thickened by 2 away
//! from the center: the solid between the spheres of radius 10 and 12 inside the cone about z with
//! its apex at the origin through the cap's rim.
use springrep::topo::Brep;

use crate::shared::thicken::{arc, line, sheet_of, thickened_sheet, turned_whole};

pub fn build() -> Result<Brep, String> {
    // the sheet is the curved face of the segment of the ball of radius 10 above z = 6, taken
    // alone, its normal away from the center
    let up = 6f64.atan2(8.);
    let segment = turned_whole(
        vec![
            line([0., 6.], [8., 6.], "base"),
            arc([0., 0.], 10., up, std::f64::consts::FRAC_PI_2 - up, "cap"),
            line([0., 10.], [0., 6.], "axis"),
        ],
        "segment",
    )?;
    thickened_sheet(&sheet_of(&segment, &["segment.cap"])?, 2., 0.)
}
