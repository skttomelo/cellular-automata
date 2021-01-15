use specs::{System, WriteStorage, ReadStorage, Join};

use crate::components::{Position, Material, MaterialType};

struct SandSystem;

impl<'a> System<'a> for SandSystem {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (ReadStorage<'a, Material>, WriteStorage<'a, Position>);

    fn run(&mut self, (mat, mut pos): Self::SystemData) {
        // TODO: figure out how to compare one position to the rest of the positions in the world
        for (mat, pos) in (&mat, &mut pos).join() {
            if mat.0 != MaterialType::Sand {
                continue;
            }

            // comparison for positions go here...
            // will more than likely use distance formula as well once the former is figured out
        }
    }
}