//! The box (−10, −10, −10)–(40, 40, 0) less everything a bull-nose mill passes through as its
//! upright axis runs from (0, 0) along x to (20, 0), a quarter turn about the upright line through
//! (20, 10) to (30, 10), and along y to (30, 30): a cylinder of radius 2 from z = −1.5 to 5, its
//! bottom corner rounded to 0.5.
use std::f64::consts::FRAC_PI_2;

use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{arc, boxed, line, outline, unite, upright, v3};

pub fn build() -> Result<Brep, String> {
    let block = boxed([-10.0, -10.0, -10.0], [40.0, 40.0, 0.0], "block")?;
    let meridian = outline(vec![
        line((0.0, -1.5), (1.5, -1.5), "bottom"),
        arc((1.5, -1.0), 0.5, -FRAC_PI_2, FRAC_PI_2, "round"),
        line((2.0, -1.0), (2.0, 5.0), "side"),
        line((2.0, 5.0), (0.0, 5.0), "top"),
        line((0.0, 5.0), (0.0, -1.5), "axis"),
    ]);
    // The path is three pieces, each swept on its own from where the mill stands as it starts.
    let first = upright(&meridian, 0.0, 0.0, "mill")?;
    let second = upright(&meridian, 20.0, 0.0, "mill")?;
    let third = upright(&meridian, 30.0, 10.0, "mill")?;
    let bend = Motion::Turn {
        origin: v3(20.0, 10.0, 0.0),
        axis: Vec3::Z,
        from: 0.0,
        to: FRAC_PI_2,
    };
    let groove = unite(&[
        &solid(&first, &Motion::Slide(v3(20.0, 0.0, 0.0)), "groove").map_err(e)?,
        &solid(&second, &bend, "groove").map_err(e)?,
        &solid(&third, &Motion::Slide(v3(0.0, 20.0, 0.0)), "groove").map_err(e)?,
    ])?;
    difference(&block, &groove).map_err(e)
}
