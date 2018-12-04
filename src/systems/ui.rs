use amethyst::{
    core::transform::Transform,
    ecs::{Read, System, WriteStorage},
    input::InputHandler,
};
use CurrentState;
use GlobalGame;

pub struct UiSystem;

impl<'s> System<'s> for UiSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        Read<'s, GlobalGame>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, g, input): Self::SystemData) {
        if g.current_state == CurrentState::GamePlay {
            return;
        }
    }
}
