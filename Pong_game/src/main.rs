use amethyst::{
    prelude::*,
    ui::{RenderUi, UiBundle},
    input::{InputBundle, StringBindings},
    core::transform::TransformBundle,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
mod pong;
use crate::pong::Pong;


// comme ca on import qu'un truc et on a tout
// This declaration will look for a file named `systems.rs` or `systems/mod.rs` and will
// insert its contents inside a module named `systems` under this scope
mod systems;



fn main() -> amethyst::Result<()> {

    // We'll put the rest of the code here.
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("assets").join("config/display.ron");
    let binding_path = app_root.join("assets").join("config/bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let assets_dir = app_root.join("assets");
    let mut world = World::new();
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(systems::BounceSystem,"collision_system", &["paddle_system", "ball_system"])
        .with(systems::WinnerSystem, "winner_system", &["ball_system"])
        .with_bundle(UiBundle::<StringBindings>::new())?
        ;

    
    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    game.run();

    Ok(())
}



