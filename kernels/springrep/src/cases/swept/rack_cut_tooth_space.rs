//! A gear blank (radius 22 about the y axis) with one tooth space cut by a rack tooth rolled
//! around it.
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::math::{Frame, Point3, Vec3};
use springrep::primitive::cylinder;
use springrep::sweep::{Motion, extrude, solid};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{polygon, v3};

pub fn build() -> Result<Brep, String> {
    let face = Frame::new(Point3::ZERO, Vec3::Y, Vec3::X).map_err(e)?;
    let blank = cylinder(&face, 22.0, 6.0, "blank").map_err(e)?;
    // Turned by φ the blank carries its pitch circle past the line x = 20 by 20φ toward −z; seen
    // from the blank the tooth slides that far along the line and turns back by φ. It is drawn
    // where it is at φ = −60°.
    let start = 20.0 * PI / 3.0;
    let section = polygon(
        &[
            (18.0, -0.842856),
            (24.0, -3.026666),
            (24.0, 3.026666),
            (18.0, 0.842856),
        ],
        "tooth",
    );
    let sketch = Frame::new(v3(0.0, -2.0, start), Vec3::Y, Vec3::X).map_err(e)?;
    let tooth = extrude(&section, &sketch, 10.0, "tooth").map_err(e)?;
    let rolled = Motion::Then(
        Box::new(Motion::Slide(v3(0.0, 0.0, -2.0 * start))),
        Box::new(Motion::Turn {
            origin: Point3::ZERO,
            axis: Vec3::Y,
            from: PI / 3.0,
            to: -PI / 3.0,
        }),
    );
    let swept = solid(&tooth, &rolled, "rack").map_err(e)?;
    difference(&blank, &swept).map_err(e)
}
