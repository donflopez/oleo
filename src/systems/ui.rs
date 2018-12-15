use amethyst::{
    core::transform::Transform,
    ecs::{Entity, Join, Read, ReadStorage, System, Write, WriteStorage},
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
        WriteStorage<'s, Ui>,
        Write<'s, GlobalGame>,
        Read<'s, Algebra>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, mut hiddens, mut uis, mut g, alg, input): Self::SystemData) {
        if g.current_state == CurrentState::GamePlay {
            return;
        }

        let tile_pos = alg.get_tile_pos(10., 16., -0.5);
        // println!("{}", tile_pos);

        for ui in (&mut uis).join() {
            let x = tile_pos[0];
            let y = tile_pos[1];

            let entity: Option<Entity> = ui.get_entity(tile_pos);

            if let Some(e) = entity {
                if !hiddens.contains(e) {
                    ui.set_sprite_value(x as usize, y as usize);
                    g.selected = ui.selected;
                    hiddens.clear();
                    hiddens.insert(e, Hidden::default());
                }
            }

            // println!("Tile pos {} with entity {:?}", tile_pos, entity);
        }

        // println!("Mouse position in plane: {}", m);
    }
}
