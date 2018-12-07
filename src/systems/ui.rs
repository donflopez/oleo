use amethyst::{
    core::transform::Transform,
    ecs::{Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};
use models::coords::Algebra;
use models::ui::Ui;
use CurrentState;
use GlobalGame;

pub struct UiSystem;

impl<'s> System<'s> for UiSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ui>,
        Read<'s, GlobalGame>,
        Read<'s, Algebra>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, ui, g, alg, input): Self::SystemData) {
        if g.current_state == CurrentState::GamePlay {
            return;
        }

        let m = alg.get_mouse_position(10.);

        // println!("Mouse position in plane: {}", m);
    }
}
