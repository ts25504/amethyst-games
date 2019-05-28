extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
};

pub struct Pong;

impl SimpleState for Pong {

}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?;

    let assets_dir = format!("{}/assets/", application_root_dir());
    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();
    Ok(())
}
