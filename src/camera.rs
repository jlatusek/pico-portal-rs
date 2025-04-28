use crate::player;
use bevy::{core_pipeline::bloom::Bloom, prelude::*};
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, update);
    }
}

fn update(
    mut camera: Single<&mut Transform, (With<Camera>, Without<player::Player>)>,
    player_transform: Single<&Transform, (With<player::Player>, Without<Camera2d>)>,
) {
    camera.translation.x = player_transform.translation.x;
    camera.translation.y = player_transform.translation.y;
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Camera::default(), Bloom::NATURAL));
}
