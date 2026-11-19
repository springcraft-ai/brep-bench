//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything the bar
//! 10 ≤ x ≤ 11, |y| ≤ 0.5, |z| ≤ 0.6 passes through as it turns about the z axis from −140° to 30°.
use springrep::boolean::difference;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, deg, sector_block, turning};

pub fn build() -> Result<Brep, String> {
    let bar = boxed([10.0, -0.5, -0.6], [11.0, 0.5, 0.6], "bar")?;
    let notch = solid(&bar, &turning(deg(-140.0), deg(30.0)), "notch").map_err(e)?;
    difference(&sector_block()?, &notch).map_err(e)
}
