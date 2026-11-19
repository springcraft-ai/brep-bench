//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a ball of
//! radius 0.8 at (10, 0, 0) on a stem leaning 20° toward +y passes through as it turns about the
//! z axis from −140° to 30°.
use springrep::boolean::{difference, union};
use springrep::math::{Frame, Vec3};
use springrep::primitive::{cylinder, sphere};
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, sector_block, turning, v3};

pub fn build() -> Result<Brep, String> {
    let lean = deg(20.0);
    let up = Frame::new(v3(10.0, 0.0, 0.0), v3(0.0, lean.sin(), lean.cos()), Vec3::X).map_err(e)?;
    let stem = cylinder(&up, 0.4, 3.0, "stem").map_err(e)?;
    let ball = sphere(&crate::shared::at([10.0, 0.0, 0.0]), 0.8, "ball").map_err(e)?;
    let tool = union(&ball, &stem).map_err(e)?;
    let swept = solid(&tool, &turning(deg(-140.0), deg(30.0)), "swing").map_err(e)?;
    difference(&sector_block()?, &swept).map_err(e)
}
