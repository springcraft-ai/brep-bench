//! The ball of radius 5 standing in a plate, the concave circle where they meet chamfered 2 out
//! across the plate at 40° to it, filling the groove.
use springrep::topo::Brep;

use crate::shared::chamfer::plate_rim_by_angle;

pub fn build() -> Result<Brep, String> {
    plate_rim_by_angle(2., 40f64.to_radians())
}
