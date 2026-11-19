//! The solid a square bar (side 2, from z = 0 to 2) fills as it is carried round the circle of
//! radius 5 while turning by −2t about its own center.
use std::f64::consts::PI;

use springrep::math::{Point3, Vec3};
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, v3};

pub fn build() -> Result<Brep, String> {
    let bar = boxed([4.0, -1.0, 0.0], [6.0, 1.0, 2.0], "bar")?;
    // turned by −3t about its own center, then carried by t about the z axis: by −2t in all
    let planet = Motion::Then(
        Box::new(Motion::Turn {
            origin: v3(5.0, 0.0, 0.0),
            axis: Vec3::Z,
            from: 0.0,
            to: -6.0 * PI,
        }),
        Box::new(Motion::Turn {
            origin: Point3::ZERO,
            axis: Vec3::Z,
            from: 0.0,
            to: 2.0 * PI,
        }),
    );
    solid(&bar, &planet, "planet").map_err(e)
}
