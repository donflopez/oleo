use amethyst::ecs::{Component, DenseVecStorage, Entity};
// use models::Tile;

pub struct Ui {
    selected: usize,
    list: Vec<Entity>,
    pub total: usize,
}

const TILES: usize = 78;

impl Ui {
    pub fn new() -> Ui {
        let list = Vec::new();

        Ui {
            selected: 0,
            list,
            total: TILES,
        }
    }

    pub fn add_tile(&mut self, tile: Entity) {
        self.list.push(tile);
    }

    pub fn get_entities(&mut self) -> &[Entity] {
        self.list.as_slice()
    }
}

impl Component for Ui {
    type Storage = DenseVecStorage<Self>;
}
