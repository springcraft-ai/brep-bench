//! The box (−8, −8, −8)–(8, 8, 0) less everything a torus (radii 3 and 1, its axis leaning 35°)
//! passes through as it slides straight down by 8.
use springrep::boolean::difference;
use springrep::math::{Frame, Vec3};
use springrep::primitive::torus;
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, deg, v3};

pub fn build() -> Result<Brep, String> {
    let stock = boxed([-8.0, -8.0, -8.0], [8.0, 8.0, 0.0], "block")?;
    let lean = deg(35.0);
    let axis =
        Frame::new(v3(0.0, 0.0, 4.0), v3(lean.sin(), 0.0, lean.cos()), Vec3::X).map_err(e)?;
    let ring = torus(&axis, 3.0, 1.0, "ring").map_err(e)?;
    let swept = solid(&ring, &Motion::Slide(v3(0.0, 0.0, -8.0)), "push").map_err(e)?;
    difference(&stock, &swept).map_err(e)
}
