use bevy::prelude::*;

pub enum Action {
    LEFT,
    RIGHT,
    JUMP,
}

#[derive(Event)]
pub struct PlayerEvent {
    pub action: Action,
}

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_control_events)
            .add_event::<PlayerEvent>();
    }
}

fn update_control_events(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_action_writer: EventWriter<PlayerEvent>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        player_action_writer.send(PlayerEvent {
            action: Action::LEFT,
        });
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        player_action_writer.send(PlayerEvent {
            action: Action::RIGHT,
        });
    }
    if keys.pressed(KeyCode::Space) || keys.pressed(KeyCode::ArrowUp) {
        player_action_writer.send(PlayerEvent {
            action: Action::JUMP,
        });
    }
    if keys.pressed(KeyCode::KeyQ) {
        app_exit_events.send(AppExit::Success);
    }
}
