use amethyst::ecs::{Component, DenseVecStorage, Entity};

// pub struct Kind {}

#[derive(Debug)]
pub struct Tile {
    // kind: Kind,
    pub kind: u8,
}

impl Tile {
    pub fn new() -> Tile {
        Tile { kind: 3 }
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Terrain {
    pub dimension_x: usize,
    pub dimension_y: usize,
    pub tiles: Vec<Vec<Entity>>,
}

impl Terrain {
    pub fn new(width: usize, heigh: usize, tiles: Vec<Vec<Entity>>) -> Terrain {
        Terrain {
            dimension_x: width,
            dimension_y: heigh,
            tiles,
        }
    }
}

impl Component for Terrain {
    type Storage = DenseVecStorage<Self>;
}
