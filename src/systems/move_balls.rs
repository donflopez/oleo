use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};
use models::{Nodite, Tile};
// use Ball;

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        ReadStorage<'s, Nodite>,
        ReadStorage<'s, Tile>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, vox, mut locals, time): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        // for (ball, local) in (&balls, &mut locals).join() {
        //     local.translate_x(ball.velocity[0] * time.delta_seconds());
        //     local.translate_y(ball.velocity[1] * time.delta_seconds());
        // }
    }
}
