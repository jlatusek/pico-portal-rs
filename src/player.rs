use bevy::prelude::*;
use num::clamp;

use crate::resolution;

const SPEED: f32 = 200.;
const JUMP: f32 = 400.;
const GRAVITY: f32 = 300.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_sprites)
            .add_systems(Startup, setup_player)
            .add_systems(Update, update_player);
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
        Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(resolution.pixel_ratio)),
        Player {},
    ));
}

fn load_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_image: Handle<Image> = asset_server.load("player-front.png");
    let sprite = Sprite::from_image(player_image);
    commands.insert_resource(PlayerResources { front: sprite });
}

fn update_player(
    keys: Res<ButtonInput<KeyCode>>,
    resolution: Res<resolution::Resolution>,
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
) {
    let (_, mut transform) = player_query.single_mut();
    let direction = (((keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight)) as i32)
        - ((keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft)) as i32))
        as f32;
    transform.translation.x += direction * time.delta_secs() * SPEED;
    let bound = resolution.screen_dimensions.x / 2.;

    transform.translation.x = clamp(transform.translation.x, -bound, bound);
}
