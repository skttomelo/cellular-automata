use specs::{Component, VecStorage};

// used to track entity position in world
#[derive(Clone, PartialEq, Debug)]
pub struct Position(pub f32, pub f32);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

// used for updating entity position
#[derive(Clone, Debug)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
    pub last_vx: f32,
    pub last_vy: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum MaterialType {
    Sand,
    Water,
    Nothing,
}

#[derive(Clone, Debug)]
pub struct Material(pub MaterialType);

impl Component for Material {
    type Storage = VecStorage<Self>;
}
