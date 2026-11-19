//! The solid the torus about (2.02, 0, 1) (radii 1 and 0.08) fills as it tumbles about two
//! nearly square axes, for t from −180° to 180°.
use springrep::topo::Brep;

use crate::shared::swept::{deg, tumble, tumbled, v3};

pub fn build() -> Result<Brep, String> {
    let motion = tumble(
        1.01,
        -1.0,
        (deg(1.0), deg(89.0), 0.0),
        deg(-180.0),
        deg(180.0),
    );
    tumbled(v3(2.02, 0.0, 1.0), 0.08, &motion)
}
