use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;
use std::collections::LinkedList;

type Coord = (f32, f32);

#[derive(Debug)]
pub struct Nodite {
    pub radius: f32,
    pub coords: Coord,
    pub connections: Vec<Coord>,
}

impl Component for Nodite {
    type Storage = DenseVecStorage<Self>;
}

pub struct Net {
    pub nodites: Vec<Nodite>,
}

fn get_base_connections(x: i8, y: i8) -> Vec<Coord> {
    let mut v: Vec<Coord> = Vec::new();
    for xx in (x - 1)..(1 + x) {
        for yy in (y - 1)..(1 + y) {
            if xx == x && yy != y {
                v.push((x as f32, yy as f32));
            }
            if yy == y && xx != x {
                v.push((xx as f32, y as f32));
            }
        }
    }

    v
}

impl Net {
    pub fn new() -> Net {
        let mut nodites: Vec<Nodite> = Vec::new();

        for i in 0..1 {
            let y = (i / 10 as u8) * 10;
            let x = (i % 10 as u8) * 10;

            println!("X: {} - Y: {}", x, y);
            nodites.push(Nodite {
                radius: 1.0,
                coords: (x as f32, y as f32),
                connections: get_base_connections(x as i8, y as i8),
            });
        }

        Net { nodites }
    }
}
