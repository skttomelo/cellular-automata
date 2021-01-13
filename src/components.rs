use specs::{Component, VecStorage};

// used to track entity position in world
#[derive(Debug)]
struct Position(f32, f32);

impl Component for Position{
    type Storage = VecStorage<Self>;
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum MaterialType {
    Sand,
    Water,
}

#[derive(Debug)]
pub struct Material;