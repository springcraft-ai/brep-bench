//! A dumbbell-shaped plate, two 10 x 10 squares joined by a neck 4 long and 2 wide, extruded 6 along
//! +z, shelled inward to a wall 1.5 thick, open at its top: the wall is every point within 1.5 of
//! the faces other than the top, so the neck is left solid and each pocket bulges toward the neck
//! in arcs of radius 1.5 about the neck's concave corners.
use springrep::edit::Wall;
use springrep::topo::Brep;

use crate::shared::shell::{prism, shelled};

pub fn build() -> Result<Brep, String> {
    let outline = [
        [0., 0.],
        [10., 0.],
        [10., 4.],
        [14., 4.],
        [14., 0.],
        [24., 0.],
        [24., 10.],
        [14., 10.],
        [14., 6.],
        [10., 6.],
        [10., 10.],
        [0., 10.],
    ];
    let names: Vec<String> = (0..outline.len()).map(|i| format!("s{i}")).collect();
    let names: Vec<&str> = names.iter().map(String::as_str).collect();
    let plate = prism(&outline, &names, 6., "plate")?;
    shelled(&plate, &["plate.end"], 1.5, Wall::Inward, "shell")
}
