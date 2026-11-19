//! The plate (−10, −10, 0)–(10, 10, 4) less everything a core drill passes through as it slides
//! along its own axis by 24: the tube between radii 1.5 and 2.5 about the line through (0, 0, 2)
//! along u = (sin 30°, 0, cos 30°), from 18 to 6 before that point.
use springrep::boolean::difference;
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::{Motion, extrude, solid};
use springrep::topo::Brep;

use crate::shared::swept::{boxed, deg, v3};
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let plate = boxed([-10.0, -10.0, 0.0], [10.0, 10.0, 4.0], "plate")?;
    let (s, c) = deg(30.0).sin_cos();
    let start = [-18.0 * s, 0.0, 2.0 - 18.0 * c];
    let annulus =
        Profile::circle(Vec2::ZERO, 5.0).with_holes(Profile::circle(Vec2::ZERO, 3.0).loops);
    let base = frame(start, [s, 0.0, c], [0.0, 1.0, 0.0])?;
    let drill = extrude(&annulus, &base, 12.0, "drill").map_err(e)?;
    let cut = solid(&drill, &Motion::Slide(v3(s, 0.0, c) * 24.0), "cut").map_err(e)?;
    difference(&plate, &cut).map_err(e)
}
