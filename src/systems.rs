use specs::{System, Entities, WriteStorage, ReadStorage, Join};

use crate::constants::{SCREEN_WIDTH, SCREEN_HEIGHT, SCALE};
use crate::components::{Position, Velocity, Material, MaterialType};

pub struct SandSystem;

impl<'a> System<'a> for SandSystem {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (Entities<'a>, ReadStorage<'a, Material>, ReadStorage<'a, Position>, WriteStorage<'a, Velocity>);

    fn run(&mut self, (entities, materials, positions, mut velocities): Self::SystemData) {
        // TODO: figure out how to compare one position to the rest of the positions in the world

        for (ent, mat, pos, vel) in (&entities, &materials, &positions, &mut velocities).join() {
            if mat.0 != MaterialType::Sand {
                continue;
            }
            
            let mut directions = [false; 3]; // if a value in the array is true then that direction is blocked

            // comparison for positions go here...
            // better version of this would prolly be having the ability to query a specific position within the world(?)
            // will more than likely use distance formula as well once the former is figured out
            for (ent_1, pos_1) in (&entities, &positions).join() {
                if ent.id() == ent_1.id() {
                    continue;
                }

                if pos.0 == pos_1.0 && pos.1 + 1.0 == pos_1.1 {
                    directions[0] = true; // there is something below the entity
                } else if pos.0 - 1.0 == pos_1.0 && pos.1 + 1.0 == pos_1.1 {
                    directions[1] = true; // there is something down-left of the entity
                } else if pos.0 + 1.0 == pos_1.0 && pos.1 + 1.0 == pos_1.1 {
                    directions[2] = true; // there is something down-right of the entity
                }
            }

            if directions[0] == false {
                vel.1 = 1.0;
            } else if directions[1] == false {
                vel.0 = -1.0;
                vel.1 = 1.0;
            } else if directions[2] == false {
                vel.0 = 1.0;
                vel.1 = 1.0;
            } else {
                vel.0 = 0.0;
                vel.1 = 0.0;
            }
        }
    }
}

// applies velocity to position and then resets velocity
pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Position>, WriteStorage<'a, Velocity>);
    fn run(&mut self, (mut positions, mut velocities): Self::SystemData) {
        for (pos, vel) in (&mut positions, &mut velocities).join() {

            if pos.1 + vel.1 >= SCREEN_HEIGHT/SCALE || pos.1 + vel.1 <= 0.0 {
                vel.1 = 0.0;
                vel.1 = 0.0;
                continue;
            }
            
            if pos.0 + vel.0 >= SCREEN_WIDTH/SCALE || pos.0 + vel.0 <= 0.0 {
                vel.1 = 0.0;
                vel.1 = 0.0;
                continue;
            }
            
            if vel.0 == 0.0 && vel.1 == 0.0 {
                continue;
            }

            pos.0 += vel.0;
            pos.1 += vel.1;
            vel.0 = 0.0;
            vel.1 = 0.0;
        }
    }
}