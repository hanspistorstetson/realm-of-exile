use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::transform::Transform;
use amethyst::core::Parent;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::ecs::Entity;
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture, Transparent,
};
use amethyst::window::ScreenDimensions;

#[derive(Default)]
pub struct Realm {}

pub struct Player;

impl Component for Player {
    type Storage = DenseVecStorage<Player>;
}

impl SimpleState for Realm {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Player>();

        let player_sprite_sheet_handle =
            load_sprite_sheet(world, "texture/pink_monster/pink_monster");
        let background_sprite_sheet_handle =
            load_sprite_sheet(world, "texture/background/background");

        let _background = init_background(world, &background_sprite_sheet_handle);
        let player = init_player(world, &player_sprite_sheet_handle);
        let _camera = init_camera(world, player);
    }
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}

fn init_background(world: &mut World, sprite_sheet: &Handle<SpriteSheet>) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_z(-10.0);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .with(Transparent)
        .named("Background")
        .build()
}

fn init_player(world: &mut World, sprite_sheet: &Handle<SpriteSheet>) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., 0.);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(transform)
        .with(Player)
        .with(sprite)
        .with(Transparent)
        .named("Player")
        .build()
}

fn init_camera(world: &mut World, parent: Entity) -> Entity {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut camera_transform = Transform::default();
    camera_transform.set_translation_z(5.0);

    world
        .create_entity()
        .with(camera_transform)
        .with(Parent { entity: parent })
        .with(Camera::standard_2d(width, height))
        .named("camera")
        .build()
}

fn load_sprite_sheet(world: &mut World, path: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("{}.png", path),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("{}.ron", path),
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_storage,
    )
}
