// use states::Ui;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::{Builder, World};
use amethyst::input::is_key_down;
use amethyst::renderer::{
    PngFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture,
    TextureMetadata, VirtualKeyCode,
};
use amethyst::utils::removal::{exec_removal, Removal};
use amethyst::{GameData, SimpleState, StateData, StateEvent, Trans};
use CurrentState;
use GlobalGame;

use models::ui::Ui;
use models::Tile;

pub struct Editor;

impl<'a, 'b> SimpleState<'a, 'b> for Editor {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;

        world.register::<Removal<CurrentState>>();

        initialise_ui(world);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;

        exec_removal(
            &world.entities(),
            &world.read_storage(),
            CurrentState::EditorMenu,
        );
    }

    fn handle_event(
        &mut self,
        data: StateData<GameData>,
        event: StateEvent,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                // Pause the game by going to the `PausedState`.
                let mut g = data.world.write_resource::<GlobalGame>();
                g.current_state = CurrentState::GamePlay;
                // println!("{:?}", g.current_state);
                return Trans::Pop;
            }
        }

        Trans::None
    }
}

fn initialise_ui(world: &mut World) -> Ui {
    let sprite_sheet_handle = load_sprite_sheet(world);

    let mut ui = Ui::new();

    for i in 0..ui.total {
        let tile_sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: i,
        };

        let mut tile_pos = Transform::default();

        tile_pos.set_xyz(((i % 10) * 16) as f32, ((i / 10) * 16) as f32, 10.);

        ui.add_tile(
            world
                .create_entity()
                .with(tile_sprite_render)
                .with(Tile::new())
                .with(tile_pos)
                .with(Removal::new(CurrentState::EditorMenu))
                .build(),
        );
    }

    ui
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/rpg_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/rpg_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}
