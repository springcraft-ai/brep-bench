//! The solid the cube (2.4, −0.6, 1.4)–(3.6, 0.6, 2.6) fills as it turns by t/4 about the
//! upright line through (2, 0, 0) and by −t about the line through (0, 0.5, 0.5) along x, for t
//! from −75° to 75°.
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, deg, tumbling};

pub fn build() -> Result<Brep, String> {
    let cube = boxed([2.4, -0.6, 1.4], [3.6, 0.6, 2.6], "cube")?;
    solid(&cube, &tumbling(deg(-75.0), deg(75.0)), "tumble").map_err(e)
}
