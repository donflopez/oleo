use amethyst::ecs::Entities;
use amethyst::renderer::{Camera, Hidden};
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entity, Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};
use models::Nodite;
use models::Terrain;
use CurrentState;
use GlobalGame;
use Paddle;

/// This system is responsible for moving all the paddles according to the user
/// provided input.
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Nodite>,
        ReadStorage<'s, Terrain>,
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Hidden>,
        Read<'s, Time>,
        Read<'s, InputHandler<String, String>>,
        WriteStorage<'s, Camera>,
        Read<'s, GlobalGame>,
    );

    fn run(
        &mut self,
        (
            paddles,
            tiles,
            terrains,
            mut entities,
            mut transforms,
            mut hiddens,
            time,
            input,
            mut camera,
            g,
        ): Self::SystemData,
    ) {
        // Iterate over all planks and move them according to the input the user
        // provided.
        if g.current_state == CurrentState::EditorMenu {
            return;
        }

        let mut cam_x = 0_f64;
        let mut cam_y = 0_f64;

        for (camera, transform) in (&camera, &mut transforms).join() {
            let opt_movement_x = input.axis_value("move_x");
            let opt_movement_y = input.axis_value("move_y");

            if let Some(movement) = opt_movement_x {
                let x = transform.translation().x;
                // println!("{:?}", transform);
                // println!("{}", movement);
                // transform.set_x(((movement + 1.0) * 40.0) as f32);
                if movement != 0.0 {
                    transform.translate_x((movement * 2.) as f32);
                }
            }

            if let Some(movement) = opt_movement_y {
                if movement != 0.0 {
                    transform.translate_y((movement * 2.) as f32);
                }
            }
            cam_x = transform.translation().x as f64;
            cam_y = transform.translation().y as f64;
            // println!("{:?}", transform);
        }

        // for (paddle, transform) in (&paddles, &mut transforms).join() {
        //     let opt_movement = match paddle.side {
        //         Side::Left => input.axis_value("left_paddle"),
        //         Side::Right => input.axis_value("right_paddle"),
        //     };

        //     if let Some(movement) = opt_movement {
        //         use ARENA_HEIGHT;
        //         // transform.translate_y(paddle.velocity * time.delta_seconds() * movement as f32);
        //         // camera.set_z(1 * movement);
        //         // We make sure the paddle remains in the arena.
        //         let paddle_y = transform.translation().y;
        //         // transform.set_y(
        //         //     paddle_y
        //         //         .max(paddle.height * 0.5)
        //         //         .min(ARENA_HEIGHT - paddle.height * 0.5),
        //         // );
        //     }
        // }

        if let Some((mouse_x, mouse_y)) = input.mouse_position() {
            for terrain in (&terrains).join() {
                let t = terrain;
                // for (tile, transform, entity) in (&tiles, &mut transforms, &entities).join() {
                // println!("{}:{}", mouse_x, mouse_y);
                let (x, y) = get_plane_pos(mouse_x, mouse_y);
                let (x, y) = ((cam_x + x) / 16., (cam_y - y) / 16.);
                let (x, y) = (x + 0.5, y + 0.5);
                // let tx = transform.translation().x as f64;
                // let ty = transform.translation().y as f64;

                // let v: Vec<Vec<Entity>> = t.tiles;

                // println!("{}:{}", x, y);
                if x > 0. && y > 0. && (x as usize) < t.dimension_x && (y as usize) < t.dimension_y
                {
                    let t_entity = t.tiles.get(y as usize).unwrap().get(x as usize).unwrap();
                    if !hiddens.contains(*t_entity) {
                        hiddens.clear();
                        hiddens.insert(*t_entity, Hidden::default());
                    }
                }

                // if tx < x && tx + 16. > x && ty < y && ty + 16. > y {
                //     hiddens.insert(entity, Hidden::default());
                // }
                // println!("{}:{}", cam_x + x, cam_y + y);
            }
        }
    }
}

fn get_plane_pos(x: f64, y: f64) -> (f64, f64) {
    // NOTE: Add dynamic screen size and z value from
    // from camera.
    let s_h = 900.;
    let s_w = 1024.;
    let a = 100.;

    let x = (2. * a * x) / s_w;
    let y = (2. * a * y) / s_h;

    (x - 100., y - 100.)
}
