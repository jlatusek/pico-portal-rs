use bevy::prelude::*;

use crate::resolution;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_sprites)
            .add_systems(Startup, setup_player);
    }
}

#[derive(Component)]
struct Player;

#[derive(Resource)]
pub struct PlayerResources {
    front: Sprite,
}

fn setup_player(
    mut commands: Commands,
    resolution: Res<resolution::Resolution>,
    player_resources: Res<PlayerResources>,
) {
    commands.spawn((
        player_resources.front.clone(),
        Transform::from_xyz(
            resolution.screen_dimensions.x / 2.0,
            resolution.screen_dimensions.y / 2.0,
            0.,
        )
        .with_scale(Vec3::splat(resolution.pixel_ratio)),
        Player {},
    ));
}

fn load_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_image: Handle<Image> = asset_server.load("player-front.png");
    let sprite = Sprite::from_image(player_image);
    commands.insert_resource(PlayerResources { front: sprite });
}
