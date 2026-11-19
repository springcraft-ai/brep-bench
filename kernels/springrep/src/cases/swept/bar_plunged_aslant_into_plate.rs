//! The plate (0, −5, 0)–(20, 5, 4) less everything a square bar passes through as it slides by 8
//! along (0.3, 0, √0.91): the prism 6 ≤ x ≤ 14, |y| + |z + 6| ≤ 1.
use springrep::boolean::difference;
use springrep::sweep::{Motion, extrude, solid};
use springrep::topo::Brep;

use crate::shared::swept::{boxed, polygon, v3};
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let plate = boxed([0.0, -5.0, 0.0], [20.0, 5.0, 4.0], "plate")?;
    // the section in (y, z), drawn on the plane x = 6 and carried 8 along x
    let diamond = polygon(
        &[(0.0, -7.0), (1.0, -6.0), (0.0, -5.0), (-1.0, -6.0)],
        "side",
    );
    let start = frame([6.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])?;
    let bar = extrude(&diamond, &start, 8.0, "bar").map_err(e)?;
    let v = v3(0.3, 0.0, 0.91f64.sqrt()) * 8.0;
    let pocket = solid(&bar, &Motion::Slide(v), "pocket").map_err(e)?;
    difference(&plate, &pocket).map_err(e)
}
