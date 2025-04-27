use bevy::{math::VectorSpace, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::resolution;

const SPEED: f32 = 2000.0;
const JUMP: f32 = 40.0;

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
        RigidBody::Dynamic,
        Collider::ball(5.0),
        Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.,
        },
        ExternalImpulse {
            impulse: Vec2::ZERO,
            torque_impulse: 0.0,
        },
        Damping {
            linear_damping: 0.5,
            angular_damping: 1.1,
        },
        Restitution::coefficient(0.7),
        Transform::from_xyz(0., 200., 0.).with_scale(Vec3::splat(resolution.pixel_ratio)),
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
    mut player_query: Query<(&mut Player, &mut Velocity, &mut ExternalImpulse, &Transform)>,
) {
    let (_, mut velocity, mut impulse, transform) = player_query.single_mut();
    let direction = (((keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight)) as i32)
        - ((keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft)) as i32))
        as f32;
    let jump = keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp);
    if jump {
        impulse.impulse = Vec2::new(0.0, 100000.0);
    }

    velocity.linvel += Vec2::new(direction * SPEED * time.delta_secs(), 0.0);
    let bound = resolution.screen_dimensions.x / 2.;
    if transform.translation.x >= bound {
        if velocity.linvel.x >= 0.0 {
            velocity.linvel.x = 0.0;
        }
    }
    if transform.translation.x <= -bound {
        if velocity.linvel.x <= 0.0 {
            velocity.linvel.x = 0.0;
        }
    }
}
