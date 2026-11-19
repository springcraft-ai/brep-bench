//! The lens common to two balls of radius 5 about (−3, 0, 0) and (3, 0, 0), each bounded by one
//! rational surface whose meridian is the weighted quadratic semicircle, not by a sphere.
use springrep::boolean::intersection;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::nurbs::nurbs_ball;

pub fn build() -> Result<Brep, String> {
    intersection(&nurbs_ball(-3., "a")?, &nurbs_ball(3., "b")?).map_err(e)
}
