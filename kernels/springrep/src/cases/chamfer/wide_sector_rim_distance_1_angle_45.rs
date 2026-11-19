//! A wide spherical sector (cos α = 5/13) of the ball of radius 5, its rim chamfered 1 down the cone at 45° to the
//! cone's side, turned toward the axis.
use springrep::topo::Brep;

use crate::shared::chamfer::sector_rim_by_angle;

pub fn build() -> Result<Brep, String> {
    sector_rim_by_angle(5. / 13., 1., 45f64.to_radians())
}
