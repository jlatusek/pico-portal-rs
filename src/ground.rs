use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_ground);
    }
}

fn setup_ground(mut commands: Commands) {
    commands.spawn(Collider::cuboid(500.0, 50.0)).insert((
        Transform::from_xyz(0.0, -200.0, 0.0),
        CollisionGroups::new(Group::GROUP_1 | Group::GROUP_2, Group::GROUP_1),
    ));
}
