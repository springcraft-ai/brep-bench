//! The box (−6, −6, −8)–(36, 26, 0) less everything an upright ring (a torus about the frame's y
//! axis) passes through as a frame riding a cubic Bézier curve carries it, turning with the
//! curve's tangent.
use springrep::boolean::difference;
use springrep::math::{Frame, Vec3};
use springrep::primitive::torus;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, carried, v3};

pub fn build() -> Result<Brep, String> {
    let stock = boxed([-6.0, -6.0, -8.0], [36.0, 26.0, 0.0], "block")?;
    let wheel = Frame::new(v3(0.0, 0.0, 0.5), Vec3::Y, Vec3::X).map_err(e)?;
    let ring = torus(&wheel, 1.5, 0.5, "ring").map_err(e)?;
    let swept = solid(&ring, &carried()?, "carried").map_err(e)?;
    difference(&stock, &swept).map_err(e)
}
