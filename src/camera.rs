use bevy::prelude::{Camera2dBundle, Commands, Component, Transform};
use bevy::utils::default;

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut command: Commands) {
    command.spawn(
        (Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        MainCamera
        )
    );
}