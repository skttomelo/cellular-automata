use specs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::components::{Material, MaterialType, Position, Velocity, Solid, Liquid};
use crate::constants::{SCALE, SCREEN_HEIGHT, SCREEN_WIDTH};

// used for sand and dirt falling
pub struct SolidsSystem;

impl<'a> System<'a> for SolidsSystem {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Solid>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (entities, solids, positions, mut velocities): Self::SystemData) {
        // TODO: figure out how to compare one position to the rest of the positions in the world

        for (ent, _, pos, vel) in (&entities, &solids, &positions, &mut velocities).join() {
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
            }
        }
    }
}

pub struct LiquidsSystem;

impl<'a> System<'a> for LiquidsSystem {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Liquid>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (entities, liquids, positions, mut velocities): Self::SystemData) {
        // TODO: figure out how to compare one position to the rest of the positions in the world

        for (ent, _, pos, vel) in (&entities, &liquids, &positions, &mut velocities).join() {
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

pub struct OverlapCorrectionSystem;

impl<'a> System<'a> for OverlapCorrectionSystem {
    type SystemData = (WriteStorage<'a, Position>, Entities<'a>);

    fn run(&mut self, (mut positions, entities): Self::SystemData) {
        for ent in (&entities).join() {
            let mut overlapping = false;
            let mut blocked_spot = [false; 5];

            let ent_pos = positions.get(ent).unwrap().clone();
            // store all cardinal directions
            for target_ent in (&entities).join() {
                if ent == target_ent {
                    continue;
                }

                let target_ent_pos = positions.get(target_ent).unwrap().clone();
                
                // overlap found
                if ent_pos == target_ent_pos {
                    overlapping = true;
                }

                // collect positions that are in cardinal directions
                if ent_pos.0 - 1.0 == target_ent_pos.0 && ent_pos.1 == target_ent_pos.1 {
                    // left
                    blocked_spot[0] = true;
                } else if ent_pos.0 + 1.0 == target_ent_pos.0 && ent_pos.1 == target_ent_pos.1 {
                    // right
                    blocked_spot[1] = true;
                } else if ent_pos.0 - 1.0 == target_ent_pos.0 && ent_pos.1 + 1.0 == target_ent_pos.1 {
                    // bottom-left
                    blocked_spot[2] = true;
                } else if ent_pos.0 == target_ent_pos.0 && ent_pos.1 + 1.0 == target_ent_pos.1 {
                    // bottom
                    blocked_spot[3] = true;
                } else if ent_pos.0 + 1.0 == target_ent_pos.0 && ent_pos.1 + 1.0 == target_ent_pos.1 {
                    // bottom-right
                    blocked_spot[4] = true;
                }
            }
            // resolution
            if overlapping == false {
                continue;
            }

            let mut ent_pos = positions.get_mut(ent).unwrap();

            if blocked_spot[0] == false {
                ent_pos.0 -= 1.0;
            } else if blocked_spot[1] == false {
                ent_pos.0 += 1.0;
            } else if blocked_spot[2] == false {
                ent_pos.0 -= 1.0;
                ent_pos.1 += 1.0;
            } else if blocked_spot[3] == false {
                ent_pos.1 += 1.0;
            } else if blocked_spot[4] == false {
                ent_pos.0 += 1.0;
                ent_pos.1 += 1.0;
            }
        }
    }
}

pub struct SolidsAndLiquidInteractionSystem;

impl<'a> System<'a> for SolidsAndLiquidInteractionSystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, Position>, ReadStorage<'a, Solid>, ReadStorage<'a, Liquid>);

    fn run(&mut self, (entities, mut positions, solids, liquids): Self::SystemData) {
        for (ent, _) in (&entities, &solids).join() {
            let temp_solid_pos = positions.get(ent).unwrap().clone();
            for (target_ent, _) in (&entities, &liquids).join() {
                let liquid_pos = positions.get_mut(target_ent).unwrap();
                if temp_solid_pos.0 == liquid_pos.0 && temp_solid_pos.1 + 1.0 == liquid_pos.1 {
                    let temp_pos = liquid_pos.clone();
                    liquid_pos.0 = temp_solid_pos.0;
                    liquid_pos.1 = temp_solid_pos.1;
                    // enclosure to drop mut borrow
                    {
                        liquid_pos
                    };
                    let mut solid_pos = positions.get_mut(ent).unwrap();
                    solid_pos.0 = temp_pos.0;
                    solid_pos.1 = temp_pos.1;
                    break;
                }
            }
        }
    }
}

// will turn dirt into grass if there is nothing above it
// if there is nothing below grass it will return to its dirt state
pub struct DirtSystem;

impl<'a> System<'a> for DirtSystem {
    type SystemData = (ReadStorage<'a, Position>, WriteStorage<'a, Material>);

    fn run(&mut self, (positions, mut materials): Self::SystemData) {
        for (pos, mat) in (&positions, &mut materials).join() {
            if mat.0 != MaterialType::Dirt && mat.0 != MaterialType::Grass {
                continue;
            }

            let mut empty_space = [true; 2]; // true means nothing is there, false means it's blocked

            for target_pos in (&positions).join() {
                if pos.0 == target_pos.0 && pos.1 - 1.0 == target_pos.1 {
                    empty_space[0] = false;
                } else if pos.0 == target_pos.0 && pos.1 + 1.0 == target_pos.1 {
                    empty_space[1] = false;
                }
            }

            if empty_space[0] == true && empty_space[1] == false {
                mat.0 = MaterialType::Grass;
            } else if mat.0 != MaterialType::Dirt {
                mat.0 = MaterialType::Dirt;
            }
        }
    }
}