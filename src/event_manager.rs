use bevy::prelude::*;

#[derive(Event)]
pub enum PlayerMoveEvent {
    LEFT,
    RIGHT,
    JUMP,
}

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_control_events)
            .add_event::<PlayerMoveEvent>();
    }
}

fn update_control_events(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_action_writer: EventWriter<PlayerMoveEvent>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        player_action_writer.send(PlayerMoveEvent::LEFT);
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        player_action_writer.send(PlayerMoveEvent::RIGHT);
    }
    if keys.pressed(KeyCode::Space) || keys.pressed(KeyCode::ArrowUp) {
        player_action_writer.send(PlayerMoveEvent::JUMP);
    }
    if keys.pressed(KeyCode::KeyQ) {
        app_exit_events.send(AppExit::Success);
    }
}
