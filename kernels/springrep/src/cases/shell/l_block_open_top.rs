//! An L-shaped plate, the polygon (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) extruded 8
//! along +z, shelled inward to a wall 1 thick, open at its top: the wall is every point within 1 of
//! its faces other than the top, so at the concave edge x = 5, y = 5 its inside is the quarter of
//! the cylinder of radius 1 about that edge.
use springrep::edit::{Wall, shell_rounded};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::shell::{faces, prism};

pub fn build() -> Result<Brep, String> {
    let plate = prism(
        &[
            [0., 0.],
            [20., 0.],
            [20., 5.],
            [5., 5.],
            [5., 15.],
            [0., 15.],
        ],
        &["front", "right", "step", "riser", "back", "left"],
        8.,
        "plate",
    )?;
    let open = faces(&plate, &["plate.end"])?;
    shell_rounded(&plate, &open, 1., Wall::Inward, "shell").map_err(e)
}
