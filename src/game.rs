use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{camera, ground, player, resolution, scene, sprites};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            resolution::ResolutionPlugin,
            sprites::SpritesPlugin,
            scene::ScenePlugin,
            player::PlayerPlugin,
            camera::CameraPlugin,
            ground::GroundPlugin,
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default());
    }
}
