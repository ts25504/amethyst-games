extern crate amethyst;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, _data: StateData<GameData>) {
        let world = _data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Paddle>();

        initialise_camera(world);
        initialise_paddles(world, sprite_sheet_handle);
    }
}

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: 1.0,
            height: 1.0,
        }
    }
}
impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

const PADDLE_HEIGHT: f32 = 16.0;
const PADDLE_WIDTH: f32 = 4.0;

fn initialise_paddles(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .build();

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Flipped::Horizontal)
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}