//! The box (−6, −6, −8)–(36, 26, 0) less everything a square key passes through as a frame
//! riding a cubic Bézier curve carries it, turning with the curve's tangent.
use springrep::boolean::difference;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, carried};

pub fn build() -> Result<Brep, String> {
    let stock = boxed([-6.0, -6.0, -8.0], [36.0, 26.0, 0.0], "block")?;
    let key = boxed([-1.5, -0.8, -1.5], [1.5, 0.8, 2.5], "key")?;
    let swept = solid(&key, &carried()?, "carried").map_err(e)?;
    difference(&stock, &swept).map_err(e)
}
