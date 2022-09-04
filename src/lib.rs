use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_ecs_tilemap::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ImageSettings::default_nearest())
            .add_plugin(TilemapPlugin)
            .add_startup_system(game_startup);
    }
}

fn game_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    commands.spawn_bundle(SpriteBundle {
        texture: texture_handle,
        ..Default::default()
    });
}
