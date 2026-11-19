//! A post of radius 3 joined to a ball of radius 5 about (0, 0, 10), the concave circle where the
//! post leaves the ball chamfered with two distances, 2 down the post and 1 on the ball, filling
//! the groove.
use springrep::boolean::union;
use springrep::primitive::{cylinder, sphere};
use springrep::topo::Brep;

use crate::shared::chamfer::rim_by_two_distances;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let post = cylinder(&at([0., 0., 0.]), 3., 10., "post").map_err(e)?;
    let ball = sphere(&at([0., 0., 10.]), 5., "ball").map_err(e)?;
    let body = union(&post, &ball).map_err(e)?;
    rim_by_two_distances(&body, (3., 6.), "post.side", 2., 1.)
}
