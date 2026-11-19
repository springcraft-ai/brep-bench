//! The crossed-forward tumbling torus's swept solid less the crossed-reverse one's.
use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::crossed;

pub fn build() -> Result<Brep, String> {
    difference(&crossed(-1.0)?, &crossed(1.0)?).map_err(e)
}
