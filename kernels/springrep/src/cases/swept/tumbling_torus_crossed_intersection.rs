//! The intersection of the crossed-forward and crossed-reverse tumbling tori's swept solids.
use springrep::boolean::intersection;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::crossed;

pub fn build() -> Result<Brep, String> {
    intersection(&crossed(-1.0)?, &crossed(1.0)?).map_err(e)
}
