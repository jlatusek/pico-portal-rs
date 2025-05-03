use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{event_manager, resolution, sprites};

const SPEED: f32 = 2000.0;
const JUMP: f32 = 100000.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, update_player);
    }
}

#[derive(Component)]
pub struct Player;

fn setup_player(
    mut commands: Commands,
    resolution: Res<resolution::Resolution>,
    sprites: Res<sprites::Sprites>,
) {
    commands.spawn((
        Sprite::from_atlas_image(
            sprites.player.image.clone(),
            TextureAtlas {
                layout: sprites.player.atlas.clone(),
                index: sprites::SpriteState::FRONT as usize,
            },
        ),
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

fn update_player(
    mut player_action_reader: EventReader<event_manager::PlayerEvent>,
    player_query: Single<(
        &mut Player,
        &mut Velocity,
        &mut ExternalImpulse,
        &Transform,
        &mut Sprite,
    )>,
    resolution: Res<resolution::Resolution>,
    time: Res<Time>,
) {
    let (_, mut velocity, mut impulse, transform, mut sprite) = player_query.into_inner();

    let mut direction = 0.;

    for action in player_action_reader.read() {
        match action.action {
            event_manager::Action::LEFT => {
                direction -= 1.;
            }
            event_manager::Action::RIGHT => {
                direction += 1.;
            }
            event_manager::Action::JUMP => {
                impulse.impulse = Vec2::new(0.0, JUMP);
            }
        };
        set_texture(&action.action, &mut sprite);
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

fn set_texture(action: &event_manager::Action, sprite: &mut Sprite) {
    if let Some(atlas) = &mut sprite.texture_atlas {
        match action {
            event_manager::Action::LEFT => {
                atlas.index = sprites::SpriteState::SIDE as usize;
                sprite.flip_x = true;
            }
            event_manager::Action::RIGHT => {
                atlas.index = sprites::SpriteState::SIDE as usize;
                sprite.flip_x = false;
            }
            event_manager::Action::JUMP => {
                atlas.index = sprites::SpriteState::JUMP as usize;
            }
        }
    }
}
