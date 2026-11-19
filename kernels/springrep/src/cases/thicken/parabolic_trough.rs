//! The parabolic cylinder z = x^2/8 for -4 <= x <= 4 and 0 <= y <= 10, a sheet, thickened by 1 to
//! its concave side, toward the focus: the strip between the parabola and its offset, extruded.
use springrep::math::Vec2;
use springrep::profile::{NamedSegment, Segment};
use springrep::topo::Brep;

use crate::shared::thicken::{line, sheet_of, swept_along_y, thickened_sheet};

pub fn build() -> Result<Brep, String> {
    // The sheet is the quadratic Bezier through (-4, 2), (0, -2), (4, 2), each (x, z), exactly the
    // parabola, swept along y: a face of the solid above it up to z = 2, taken alone. Its normal
    // points away from the focus, so the focus is on its back.
    let parabola = Segment::Bezier {
        ctrl: vec![Vec2::new(-4., 2.), Vec2::new(0., -2.), Vec2::new(4., 2.)],
        weights: vec![1., 1., 1.],
    };
    let trough = swept_along_y(
        vec![
            NamedSegment::new(parabola, "sheet"),
            line([4., 2.], [-4., 2.], "top"),
        ],
        10.,
        "trough",
    )?;
    thickened_sheet(&sheet_of(&trough, &["trough.sheet"])?, 0., 1.)
}
