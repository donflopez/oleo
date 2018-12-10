use amethyst::{
    core::transform::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    renderer::Hidden,
};
use models::coords::Algebra;
use models::ui::Ui;
use CurrentState;
use GlobalGame;

pub struct UiSystem;

impl<'s> System<'s> for UiSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Hidden>,
        ReadStorage<'s, Ui>,
        Read<'s, GlobalGame>,
        Read<'s, Algebra>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, mut hiddens, uis, g, alg, input): Self::SystemData) {
        if g.current_state == CurrentState::GamePlay {
            return;
        }

        let tile_pos = alg.get_tile_pos(10., 16., -0.5);
        // println!("{}", tile_pos);

        for ui in (&uis).join() {
            let x = tile_pos[0];
            let y = tile_pos[1];

            let entity = ui.get_entity(tile_pos);

            if let Some(e) = entity {
                if !hiddens.contains(e) {
                    hiddens.clear();
                    hiddens.insert(e, Hidden::default());
                }
            }

            // println!("Tile pos {} with entity {:?}", tile_pos, entity);
        }

        // println!("Mouse position in plane: {}", m);
    }
}
