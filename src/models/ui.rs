use amethyst::core::nalgebra::Vector2;
use amethyst::ecs::{Component, DenseVecStorage, Entity};
// use models::Tile;

pub struct Ui {
    selected: usize,
    list: Vec<Vec<Entity>>,
    pub total: usize,
    grid_dim: (usize, usize),
    last_added: (usize, usize),
}

const TILES: usize = 78;

impl Ui {
    pub fn new(w: usize, h: usize) -> Ui {
        let mut list = Vec::new();

        for _ in 0..h {
            list.push(Vec::new())
        }

        Ui {
            selected: 0,
            list,
            total: TILES,
            grid_dim: (w, h),
            last_added: (0, 0),
        }
    }

    pub fn add_tile(&mut self, (x, y): (usize, usize), tile: Entity) {
        self.list[y].push(tile);
    }

    pub fn get_entity(&self, pos: Vector2<f32>) -> Option<Entity> {
        let x = pos[0] as usize;
        let y = pos[1] as usize;

        if self.list.len() > y && self.list[y].len() > x {
            return Some(self.list[y][x]);
        }

        None
    }
}

impl Component for Ui {
    type Storage = DenseVecStorage<Self>;
}
