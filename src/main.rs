use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use amethyst::input::{InputBundle, StringBindings};

mod realm;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let config = app_root.join("config");
    let binding_path = config.join("bindings.ron");
    let display_path = config.join("display.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_path)
                        .with_clear([0.541, 0.439, 0.859, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(input_bundle)?
        .with(
            systems::PlayerMovementSystem,
            "player_movement_system",
            &["input_system"],
        );

    let mut game = Application::new(resources, realm::Realm::default(), game_data)?;
    game.run();

    Ok(())
}
