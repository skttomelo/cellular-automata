use specs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::components::{Material, MaterialType, Position, Velocity};
use crate::constants::{SCALE, SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct SandSystem;

impl<'a> System<'a> for SandSystem {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Material>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

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
                vel.vy = 1.0;
            } else if directions[1] == false {
                vel.vx = -1.0;
                vel.vy = 1.0;
            } else if directions[2] == false {
                vel.vx = 1.0;
                vel.vy = 1.0;
            } else {
                vel.vx = 0.0;
                vel.vy = 0.0;
            }
        }
    }
}

pub struct WaterSystem;

impl<'a> System<'a> for WaterSystem {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Material>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (entities, materials, positions, mut velocities): Self::SystemData) {
        // TODO: figure out how to compare one position to the rest of the positions in the world

        for (ent, mat, pos, vel) in (&entities, &materials, &positions, &mut velocities).join() {
            if mat.0 != MaterialType::Water {
                continue;
            }

            let mut directions = [false; 5]; // if a value in the array is true then that direction is blocked

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
                } else if pos.0 - 1.0 == pos_1.0 && pos.1 == pos_1.1 {
                    directions[3] = true; // there is something left of the entity
                } else if pos.0 + 1.0 == pos_1.0 && pos.1 == pos_1.1 {
                    directions[4] = true; // there is something right of the entity
                }
            }

            if directions[0] == false {
                vel.vy = 1.0;
            } else if directions[1] == false && directions[3] == false {
                // this prevents diagonal movement when something is blocking the immediate left if either one is not false
                vel.vx = -1.0;
                vel.vy = 1.0;
            } else if directions[2] == false && directions[4] == false {
                // this prevents diagonal movement when something is blocking the immediate right if either one is not false
                vel.vx = 1.0;
                vel.vy = 1.0;
            } else if (directions[3] == false && directions[4] == true) || (directions[3] == false && vel.last_vx < 1.0) {
                vel.vx = -1.0;
            } else if directions[4] == false {
                vel.vx = 1.0;
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
            if pos.1 + vel.vy >= SCREEN_HEIGHT / SCALE || pos.1 + vel.vy < 0.0 {
                vel.vx = 0.0;
                vel.vy = 0.0;
                continue;
            }

            if pos.0 + vel.vx >= SCREEN_WIDTH / SCALE || pos.0 + vel.vx < 0.0 {
                vel.vx = 0.0;
                vel.vy = 0.0;
                continue;
            }

            if vel.vx == 0.0 && vel.vy == 0.0 {
                continue;
            }

            pos.0 += vel.vx;
            pos.1 += vel.vy;
            vel.last_vx = vel.vx;
            vel.last_vy = vel.vy;
            vel.vx = 0.0;
            vel.vy = 0.0;
        }
    }
}

// I'm just going to delete the ones that overlap.
// Obviously this isn't the best solution, but it's the quickest to implement.
pub struct OverlapCorrectionSystem;

impl<'a> System<'a> for OverlapCorrectionSystem {
    type SystemData = (ReadStorage<'a, Position>, Entities<'a>);

    fn run(&mut self, (positions, entities): Self::SystemData) {
        for (pos, ent) in (&positions, &entities).join() {
            // store all cardinal directions
            for pos_1 in (&positions).join() {
                // collect positions that are in cardinal directions
            }
            // resolution
            
        }
    }
}