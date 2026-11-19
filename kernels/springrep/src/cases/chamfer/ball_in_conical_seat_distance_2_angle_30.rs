//! The ball of radius 5 standing in a conical seat, the concave circle where they meet chamfered
//! 2 up the seat's cone at 30° to it, filling the groove.
use springrep::topo::Brep;

use crate::shared::chamfer::conical_seat_by_angle;

pub fn build() -> Result<Brep, String> {
    conical_seat_by_angle(2., 30f64.to_radians())
}
