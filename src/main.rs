use bevy::{prelude::*, window::PresentMode};
use miniworld::GamePlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("MiniWorld"),
            width: 800.,
            height: 600.,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
