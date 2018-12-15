use amethyst::assets::AssetLoaderSystemData;
use amethyst::assets::Handle;
use amethyst::controls::FlyControlTag;
use amethyst::core::nalgebra::UnitQuaternion;
use amethyst::renderer::Light;
use amethyst::renderer::Material;
use amethyst::renderer::MaterialDefaults;
use amethyst::renderer::Mesh;
use amethyst::renderer::PosNormTangTex;
use amethyst::renderer::Shape::Sphere;
use amethyst::renderer::SunLight;
use amethyst::ui::UiCreator;
use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        nalgebra::{Vector3, Vector4},
        transform::Transform,
    },
    ecs::prelude::{Entity, World},
    input::is_key_down,
    prelude::*,
    renderer::{
        Camera, DrawFlat, HideHierarchySystem, PngFormat, PosColorNorm, Projection, Rgba, Shape,
        SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
        VirtualKeyCode,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};
use models::{Cursor, Terrain, Tile};
use models::{Net, Nodite};
use states::editor::Editor;
use systems::ScoreText;
use CurrentState;
use GlobalGame;
use {Ball, Paddle, Side};
use {ARENA_HEIGHT, ARENA_WIDTH};

const MAP_WIDTH: usize = 64;
const MAP_HEIGHT: usize = 64;

pub struct Pong;

impl<'a, 'b> SimpleState<'a, 'b> for Pong {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;
        use audio::initialise_audio;
        // Load the spritesheet necessary to render the graphics.
        // `spritesheet` is the layout of the sprites on the image;
        // `texture` is the pixel data.
        let sprite_sheet_handle = load_sprite_sheet(world);
        // let line = Bresenham3d::new((0, 0, 0), (2, 2, 0));

        // println!("{:?}", Bresenham3d::new((0, 0, 0), (2, 2, 0)));
        // Setup our game.
        // initialise_paddles(world, sprite_sheet_handle.clone());
        // initialise_ball(world, sprite_sheet_handle.clone());
        initialise_terrain(world);
        initialise_cursor(world);
        initialise_camera(world);
        initialise_audio(world);
        initialise_score(world);
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

                g.current_state = CurrentState::EditorMenu;

                return Trans::Push(Box::new(Editor));
            }
        }

        Trans::None
    }
}

fn initialise_cursor(world: &mut World) {
    let sprite_sheet_handle = load_cursor_sprite_sheet(world);
    let mut trans = Transform::default();

    trans.set_xyz(0., 0., 10.);

    world
        .create_entity()
        .with(SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 0,
        })
        .with(Cursor {})
        .with(trans)
        .build();
}

fn initialise_tiles(world: &mut World) -> Vec<Vec<Entity>> {
    let mut tiles = Vec::new();
    let sprite_sheet_handle = load_sprite_sheet(world);

    for y in 0..MAP_HEIGHT {
        let mut col = Vec::new();

        for x in 0..MAP_WIDTH {
            let tile_sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet_handle.clone(),
                // sprite_number: (y + x) / 10, // Change in a near future
                sprite_number: 0,
            };
            let mut tile_pos = Transform::default();
            tile_pos.set_xyz((x * 16) as f32, (y * 16) as f32, 0.0);

            // println!("{:?}", tile_pos.scale());

            // tile_pos.set_scale(0.01, 0.01, 0.01);
            // let tile = Tile::new();
            // println!("{}:{}", x, y);
            col.push(
                world
                    .create_entity()
                    .with(tile_sprite_render)
                    .with(Tile::new())
                    .with(tile_pos)
                    .build(),
            );
        }

        tiles.push(col);
    }

    tiles
}

fn initialise_terrain(world: &mut World) {
    let tiles = initialise_tiles(world);

    world
        .create_entity()
        .with(Terrain::new(MAP_WIDTH, MAP_HEIGHT, tiles))
        .build();
}

fn load_mesh(world: &mut World) -> (Handle<Mesh>, Material) {
    let (mesh, mat) = {
        let mesh = world.exec(|loader: AssetLoaderSystemData<Mesh>| {
            loader.load_from_data(
                Shape::Plane(Some((1, 1))).generate::<Vec<PosNormTangTex>>(Some((2.0, 1.0, 1.0))),
                (),
            )
        });

        let (albedo, roughness, metallic) = world.exec(|loader: AssetLoaderSystemData<Texture>| {
            (
                loader.load_from_data([1.0, 1.0, 1.0, 1.0].into(), ()),
                loader.load_from_data([0.5, 0.5, 0.5, 1.0].into(), ()),
                loader.load_from_data([1.0, 1.0, 1.0, 1.0].into(), ()),
            )
        });

        let mat_defaults = world.read_resource::<MaterialDefaults>().0.clone();

        let mtl = Material {
            albedo: albedo.clone(),
            roughness,
            metallic,
            ..mat_defaults.clone()
        };

        (mesh, mtl)
    };

    (mesh, mat)
}

fn load_lights() -> (Light, Transform) {
    let light = SunLight {
        ang_rad: 2.0,
        color: [1.0, 1.0, 1.0, 1.0].into(),
        intensity: 6.0,
        direction: [0.0, 0.0, 0.0],
    }
    .into();

    let mut transform = Transform::default();
    transform.set_xyz(0.0, 0.0, -10.0);

    (light, transform)
}

fn load_cursor_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/ui_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/ui_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
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

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    // transform.set_z(1.0);

    transform.set_position([50.0, 50.0, 100.0].into());
    transform.set_rotation(UnitQuaternion::identity());
    // world
    //     .create_entity()
    //     .with(FlyControlTag)
    //     .with(Camera::from(Projection::perspective(
    //         1.33333,
    //         std::f32::consts::FRAC_PI_2,
    //     )))
    //     .with(transform)
    //     .build();

    world
        .create_entity()
        .with(Camera::from(Projection::perspective(
            1.,
            std::f32::consts::FRAC_PI_2,
        )))
        .with(transform)
        .build();
}

/// Initialises one paddle on the left, and one paddle on the right.
fn initialise_paddles(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    use {PADDLE_HEIGHT, PADDLE_VELOCITY, PADDLE_WIDTH};

    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the paddles.
    let y = (ARENA_HEIGHT - PADDLE_HEIGHT) / 2.0;
    left_transform.set_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // Assign the sprites for the paddles
    let sprite_render_left = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    let sprite_render_right = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    // Create a left plank entity.
    world
        .create_entity()
        .with(sprite_render_left)
        .with(Paddle {
            side: Side::Left,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            velocity: PADDLE_VELOCITY,
        })
        .with(left_transform)
        .build();

    // Create right plank entity.
    world
        .create_entity()
        .with(sprite_render_right)
        .with(Paddle {
            side: Side::Right,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            velocity: PADDLE_VELOCITY,
        })
        .with(right_transform)
        .build();
}

/// Initialises one ball in the middle-ish of the arena.
fn initialise_ball(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    // use {ARENA_HEIGHT, ARENA_WIDTH, BALL_RADIUS, BALL_VELOCITY_X, BALL_VELOCITY_Y};

    // Create the translation.
    let mut local_transform = Transform::default();
    local_transform.set_xyz(1.0, 1.0, 1.0);

    // Assign the sprite for the ball
    // let sprite_render = SpriteRender {
    //     sprite_sheet: sprite_sheet_handle,
    //     sprite_number: 1, // ball is the second sprite on the sprite_sheet
    //     flip_horizontal: false,
    //     flip_vertical: false,
    // };

    let net = Net::new();
    for nodite in net.nodites.iter() {
        let mut trf = Transform::default();
        let (x, y) = nodite.coords;

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 1, // ball is the second sprite on the sprite_sheet
        };

        trf.set_xyz(x, y, 0.0);
        // println!("{:?}", nodite);

        let n = Nodite {
            radius: 1.0,
            coords: (x, y),
            connections: Vec::new(),
        };

        world
            .create_entity()
            .with(sprite_render)
            .with(n)
            .with(trf)
            .build();

        // break;
    }

    let (mesh, material) = load_mesh(world);

    world
        .create_entity()
        .with(local_transform)
        .with(mesh)
        .with(material)
        .build();

    let (light, trans) = load_lights();

    world.create_entity().with(trans).with(light).build();
    // world
    //     .create_entity()
    //     .with(sprite_render.clone())
    //     .with(Ball {
    //         radius: BALL_RADIUS,
    //         velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
    //     })
    //     .with(local_transform.clone())
    //     .build();

    // let mut loct = local_transform.clone();
    // loct.translate_x(2.0);

    // world
    //     .create_entity()
    //     .with(sprite_render)
    //     .with(Ball {
    //         radius: BALL_RADIUS,
    //         velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
    //     })
    //     .with(loct)
    //     .build();

    // let mut local_transform2 = Transform::default();
    // local_transform2.set_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    // world
    //     .create_entity()
    //     // .with()
    //     .with(Nodite {
    //         radius: BALL_RADIUS,
    //     })
    //     .with(local_transform2)
    //     .build();
}

fn initialise_score(world: &mut World) {
    world.exec(|mut creator: UiCreator| {
        creator.create("ui/editor.ron", ());
    });

    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );
    let p1_transform = UiTransform::new(
        "gallery".to_string(),
        Anchor::TopMiddle,
        -50.,
        -50.,
        1.,
        200.,
        50.,
        0,
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            50.,
        ))
        .build();

    world.add_resource(ScoreText { p1_score });
}
