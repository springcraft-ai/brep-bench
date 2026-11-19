//! The block (0, −5, 0)–(20, 5, 10) less everything a V wheel passes through as it slides along
//! the x axis by 36: the solid of revolution about the line through (−8, 0, 14) along y of the
//! triangle (0, 8), (−3 tan 30°, 11), (3 tan 30°, 11) in (y, z) on the plane x = −8.
use std::f64::consts::TAU;

use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::sweep::{Motion, revolve, solid};
use springrep::topo::Brep;

use crate::shared::swept::{boxed, polygon, v3};
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let block = boxed([0.0, -5.0, 0.0], [20.0, 5.0, 10.0], "block")?;
    let t = 3.0 * 30f64.to_radians().tan();
    // sketch (y, z) on the plane x = −8, which holds the wheel's axis z = 14
    let plane = frame([-8.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])?;
    let triangle = polygon(&[(0.0, 8.0), (t, 11.0), (-t, 11.0)], "wheel");
    let wheel = revolve(
        &triangle,
        &plane,
        v3(-8.0, 0.0, 14.0),
        Vec3::Y,
        TAU,
        "wheel",
    )
    .map_err(e)?;
    let groove = solid(&wheel, &Motion::Slide(v3(36.0, 0.0, 0.0)), "groove").map_err(e)?;
    difference(&block, &groove).map_err(e)
}
