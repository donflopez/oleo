use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, Write};
use amethyst::input::InputHandler;
use amethyst::renderer::{Camera, ScreenDimensions};
use models::coords::Algebra;

pub struct AlgebraSystem;

impl<'s> System<'s> for AlgebraSystem {
    type SystemData = (
        Write<'s, Algebra>,
        Read<'s, InputHandler<String, String>>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut alg, input, screen, camera, transforms): Self::SystemData) {
        for (_, transform) in (&camera, &transforms).join() {
            alg.set_camera(transform);
        }

        if let Some((x, y)) = input.mouse_position() {
            alg.set_mouse(x as f32, y as f32);
        }

        alg.set_screen(&screen);
    }
}
