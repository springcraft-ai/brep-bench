//! A spherical sector (cos α = 3/5) of the ball of radius 5, its rim chamfered 3 down the cone at 30° to the
//! cone's side, turned toward the axis.
use springrep::topo::Brep;

use crate::shared::chamfer::sector_rim_by_angle;

pub fn build() -> Result<Brep, String> {
    sector_rim_by_angle(3. / 5., 3., 30f64.to_radians())
}
