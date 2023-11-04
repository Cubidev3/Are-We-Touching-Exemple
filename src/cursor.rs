use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Camera, Color, Commands, Component, GlobalTransform, Query, Res, ResMut, Resource, Transform, With};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::utils::default;

use bevy::window::{PrimaryWindow, Window};

#[derive(Component)]
pub struct Cursor {
    pub click_area: Vec2
}

pub fn spawn_cursor(mut commands: Commands) {
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.15, 0.75, 0.45),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
        },
        Cursor {
            click_area: Vec2::new(10.0, 10.0)
        }
    ));
}

pub fn move_cursor(coords: Res<MyWorldCoords>, mut cursor_query: Query<&mut Transform, With<Cursor>>) {
    let mut tranform = cursor_query.single_mut();

    let Vec2{ x, y } = coords.0;
    tranform.translation.x = x;
    tranform.translation.y = y;
}

use crate::camera::MainCamera;

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct MyWorldCoords(pub Vec2);

pub fn my_cursor_system<'a>(
    mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&'a Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&'a Camera, &'a GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
        eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}