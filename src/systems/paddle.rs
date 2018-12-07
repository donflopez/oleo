use amethyst::renderer::{Camera, Hidden};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};
use models::coords::Algebra;
use models::Terrain;
use CurrentState;
use GlobalGame;

/// This system is responsible for moving all the paddles according to the user
/// provided input.
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        ReadStorage<'s, Terrain>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Hidden>,
        Read<'s, InputHandler<String, String>>,
        ReadStorage<'s, Camera>,
        Read<'s, GlobalGame>,
        Read<'s, Algebra>,
    );

    fn run(
        &mut self,
        (terrains, mut transforms, mut hiddens, input, camera, g, alg): Self::SystemData,
    ) {
        if g.current_state == CurrentState::EditorMenu {
            return;
        }

        for (_, transform) in (&camera, &mut transforms).join() {
            let opt_movement_x = input.axis_value("move_x");
            let opt_movement_y = input.axis_value("move_y");

            if let Some(movement) = opt_movement_x {
                if movement != 0.0 {
                    transform.translate_x((movement * 2.) as f32);
                }
            }

            if let Some(movement) = opt_movement_y {
                if movement != 0.0 {
                    transform.translate_y((movement * 2.) as f32);
                }
            }
        }

        for terrain in (&terrains).join() {
            let t = terrain;

            let tile_pos = alg.get_tile_pos(0., 16., -0.5);
            let x = tile_pos[0];
            let y = tile_pos[1];

            if x > 0. && y > 0. && (x as usize) < t.dimension_x && (y as usize) < t.dimension_y {
                let t_entity = t.tiles.get(y as usize).unwrap().get(x as usize).unwrap();
                if !hiddens.contains(*t_entity) {
                    hiddens.clear();
                    hiddens.insert(*t_entity, Hidden::default());
                }
            }
        }
    }
}
