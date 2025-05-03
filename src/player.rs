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
        // ActiveEvents::COLLISION_EVENTS,
        ActiveHooks::FILTER_INTERSECTION_PAIR,
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
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
        Player {},
    ));
}

fn update_player(
    rapier_context: ReadRapierContext,
    mut player_action_reader: EventReader<event_manager::PlayerMoveEvent>,
    player_query: Single<(
        &mut Player,
        &mut Velocity,
        &mut ExternalImpulse,
        &Transform,
        &mut Sprite,
        &Collider,
    )>,
    resolution: Res<resolution::Resolution>,
    time: Res<Time>,
) {
    let (_, mut velocity, mut impulse, transform, mut sprite, collider) = player_query.into_inner();

    let mut direction = 0.;

    for move_event in player_action_reader.read() {
        match move_event {
            event_manager::PlayerMoveEvent::LEFT => {
                direction -= 1.;
            }
            event_manager::PlayerMoveEvent::RIGHT => {
                direction += 1.;
            }
            event_manager::PlayerMoveEvent::JUMP => {
                impulse.impulse = Vec2::new(0.0, JUMP);
            }
        };
        set_texture(&move_event, &mut sprite);
    }
    let filter = QueryFilter::only_fixed()
        .groups(CollisionGroups::new(Group::GROUP_1, Group::GROUP_2));
    rapier_context.single().intersections_with_shape(
        transform.translation.truncate(),
        0.0,
        &collider,
        filter,
        |entity| {
            // This closure is called for each intersecting entity
            println!("Collision detected with entity: {:?}", entity);
            true // Return true to continue checking other intersections
        },
    );

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

fn set_texture(move_event: &event_manager::PlayerMoveEvent, sprite: &mut Sprite) {
    if let Some(atlas) = &mut sprite.texture_atlas {
        match move_event {
            event_manager::PlayerMoveEvent::LEFT => {
                atlas.index = sprites::SpriteState::SIDE as usize;
                sprite.flip_x = true;
            }
            event_manager::PlayerMoveEvent::RIGHT => {
                atlas.index = sprites::SpriteState::SIDE as usize;
                sprite.flip_x = false;
            }
            event_manager::PlayerMoveEvent::JUMP => {
                atlas.index = sprites::SpriteState::JUMP as usize;
            }
        }
    }
}
