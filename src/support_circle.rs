use are_we_touching::dim2::convex::Convex2D;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Color, Commands, Component, default, Query, Transform, With, Without};
use bevy::sprite::{Sprite, SpriteBundle};
use cubi_vectors::vector2::Vector2;
use crate::cursor::Cursor;
use crate::test_rect::{create_rectangle_collision_shape, RectangleCollider2D};

#[derive(Component)]
pub struct SupportRect;

pub fn spawn_support_rect(mut commands: Commands) {
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.1, 0.7, 0.4),
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
        },
        SupportRect
    ));
}

pub fn update_translation_support_rect(mut support_query: Query<&mut Transform, With<SupportRect>>, rectangles_query: Query<(&Transform, &RectangleCollider2D), Without<SupportRect>>, cursor_query: Query<&Transform, (With<Cursor>, Without<SupportRect>)>) {
    let (mut support_transform) = support_query.single_mut();
    let (rect_transform, RectangleCollider2D { extents}) = rectangles_query.single();
    let (cursor_transform) = cursor_query.single();

    let shape = create_rectangle_collision_shape(rect_transform.translation.truncate(), extents.clone(), rect_transform.rotation.z);
    let direction = Vector2::xy(cursor_transform.translation.x - rect_transform.translation.x, cursor_transform.translation.y - rect_transform.translation.y);

    if let Some(Vector2 { x, y }) = shape.support(&direction) {
        support_transform.translation.x = x;
        support_transform.translation.y = y;
    } else {
        support_transform.translation.x = 1000.0;
    }
}