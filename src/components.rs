use specs::{Component, VecStorage};

// used to track entity position in world
#[derive(Clone, Debug)]
pub struct Position(pub f32, pub f32);

impl Component for Position{
    type Storage = VecStorage<Self>;
}

// used for updating entity position
#[derive(Clone, Debug)]
pub struct Velocity(pub f32, pub f32);

impl Component for Velocity{
    type Storage = VecStorage<Self>;
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum MaterialType {
    Sand,
    Water,
}

#[derive(Clone, Debug)]
pub struct Material(pub MaterialType);

impl Component for Material{
    type Storage = VecStorage<Self>;
}