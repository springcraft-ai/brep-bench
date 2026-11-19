//! A C-shaped bracket, two arms 40 long joined by a back, with a hole of radius 4 drilled down
//! through its upper arm and no further: from the arm's top until it comes out of the material.
use springrep::boolean::difference;
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::{Meet, extrude, extrude_to_next};
use springrep::topo::Brep;

use crate::shared::{at, e, frame};

pub fn build() -> Result<Brep, String> {
    // the section in (x, z), drawn on the plane y = 10 facing −y so the frame's y is +z, and
    // extruded 20 along −y to y = −10
    let xz = frame([0., 10., 0.], [0., -1., 0.], [1., 0., 0.])?;
    let p = |x: f64, z: f64| Vec2::new(x, z);
    let section = Profile::polygon(
        &[
            p(0., 0.),
            p(40., 0.),
            p(40., 6.),
            p(6., 6.),
            p(6., 24.),
            p(40., 24.),
            p(40., 30.),
            p(0., 30.),
        ],
        &[
            "bottom",
            "lower_end",
            "lower_top",
            "back_face",
            "upper_under",
            "upper_end",
            "top",
            "back",
        ],
    );
    let bracket = extrude(&section, &xz, 20., "bracket").map_err(e)?;
    // the disk on the top, z = 30, about (28, 0), drilled down (against +z) out of the upper arm
    let disk = Profile::circle(Vec2::ZERO, 8.);
    let top = at([28., 0., 30.]);
    let drill = extrude_to_next(&disk, &top, true, &bracket, Meet::OutOf, "hole").map_err(e)?;
    difference(&bracket, &drill).map_err(e)
}
