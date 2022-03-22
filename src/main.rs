use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::app_root_dir,
};
use amethyst::shred::RunWithPool;
use amethyst::utils::{application_dir, application_root_dir};

pub struct PikachuVolleyball;

impl SimpleState for PikachuVolleyball {}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let config_display_path = config_dir.join("display.ron");
    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(config_display_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
        )?
        .with_bundle(TransformBundle::new())?;
    let mut game = Application::new(assets_dir, PikachuVolleyball, game_data)?;
    game.run();
    Ok(())
}
