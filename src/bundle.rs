use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::DispatcherBuilder,
};

use systems::{
    AlgebraSystem, BounceSystem, MoveBallsSystem, TerrainSystem, UiSystem, WinnerSystem,
};

/// A bundle is a convenient way to initialise related resources, components and systems in a
/// world. This bundle prepares the world for a game of pong.
pub struct OleoBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for OleoBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(AlgebraSystem, "algebra_system", &["input_system"]);
        builder.add(
            TerrainSystem,
            "terrain_system",
            &["input_system", "algebra_system"],
        );
        builder.add(MoveBallsSystem, "ball_system", &[]);
        builder.add(
            BounceSystem,
            "collision_system",
            &["terrain_system", "ball_system"],
        );
        builder.add(
            WinnerSystem,
            "winner_system",
            &["terrain_system", "ball_system"],
        );
        builder.add(UiSystem, "ui_system", &["input_system"]);
        // builder.add(
        //     HideHierarchySystem::default(),
        //     "parent_hierarchy_system",
        //     &[],
        // );
        Ok(())
    }
}
