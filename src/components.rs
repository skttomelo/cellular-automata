use specs::{Component, VecStorage};

// used to track entity position in world
#[derive(Debug)]
pub struct Position(pub f32, pub f32);

impl Component for Position{
    type Storage = VecStorage<Self>;
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum MaterialType {
    Sand,
    // Water, // not implemented atm
}

#[derive(Debug)]
pub struct Material(pub MaterialType);

impl Component for Material{
    type Storage = VecStorage<Self>;
}