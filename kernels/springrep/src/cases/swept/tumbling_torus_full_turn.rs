//! The solid the torus about (3, 0, 2) (radii 1 and 0.16) fills as it tumbles about two skew
//! axes, for t from −180° to 180°.
use springrep::topo::Brep;

use crate::shared::swept::{deg, tumble, tumbled, v3};

pub fn build() -> Result<Brep, String> {
    let motion = tumble(
        0.375,
        -1.0,
        (deg(30.0), deg(20.0), 0.7071),
        deg(-180.0),
        deg(180.0),
    );
    tumbled(v3(3.0, 0.0, 2.0), 0.16, &motion)
}
