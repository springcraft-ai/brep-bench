//! The solid a regular tetrahedron with corners (5, 1, 1), (5, −1, −1), (3, 1, −1) and (3, −1,
//! 1) fills as it is screwed about the z axis, rising 5 a turn, from 0° to 540°.
use springrep::boolean::{difference, half_space};
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, deg, screwing, v3};

pub fn build() -> Result<Brep, String> {
    // the cube (3, −1, −1)–(5, 1, 1) with its other four corners cut off, each by the plane
    // through the three tetrahedron corners beside it
    let center = v3(4.0, 0.0, 0.0);
    let mut tetrahedron = boxed([3.0, -1.0, -1.0], [5.0, 1.0, 1.0], "cube")?;
    for k in [
        v3(-1.0, -1.0, -1.0),
        v3(-1.0, 1.0, 1.0),
        v3(1.0, -1.0, 1.0),
        v3(1.0, 1.0, -1.0),
    ] {
        let corner = half_space(&tetrahedron, center + k / 3.0, k).map_err(e)?;
        tetrahedron = difference(&tetrahedron, &corner).map_err(e)?;
    }
    solid(&tetrahedron, &screwing(5.0, 0.0, deg(540.0)), "screw").map_err(e)
}
