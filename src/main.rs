use bevy::prelude::*;

mod game;
mod resolution;

fn main() {
    println!("Hello, from new pico-portal!");
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("pico-portal"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(512.0, 512.0).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    mode: AssetMode::Processed,
                    file_path: "assets".to_string(),
                    processed_file_path: "assets_imported".to_string(),
                    ..Default::default()
                }),
            game::GamePlugin,
        ))
        .run();
}
