//! The solid the torus about (3, 0, 2) (radii 1 and 0.015) fills as it tumbles about two skew
//! axes, for t from −75° to 75°.
use springrep::topo::Brep;

use crate::shared::swept::{deg, tumbled, tumbling, v3};

pub fn build() -> Result<Brep, String> {
    tumbled(v3(3.0, 0.0, 2.0), 0.015, &tumbling(deg(-75.0), deg(75.0)))
}
