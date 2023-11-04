use are_we_touching::dim2::gjk::convex_shapes_overlap;
use are_we_touching::dim2::shapes::rectangle::Rectangle2D;
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{Color, Commands, Component, default, Query, Transform };
use bevy::sprite::{Sprite, SpriteBundle};
use cubi_vectors::vector2::Vector2;
use crate::cursor::Cursor;

#[derive(Component)]
pub struct RectangleCollider2D {
    pub extents: Vec2
}

const COLLIDING_COLOR: Color = Color::rgb(0.75, 0.15, 0.05);
const NOT_COLLIDING_COLOR: Color = Color::rgb(0.05, 0.15, 0.75);

pub fn spawn_rectangles(mut commands: Commands) {
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: NOT_COLLIDING_COLOR,
            custom_size: Some(Vec2::new(20.0, 40.0)),
            ..default()
            },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)).with_rotation(Quat::from_rotation_z(230f32.to_radians())),
        ..default()
        },
        RectangleCollider2D {
            extents: Vec2::new(10.0, 20.0)
        }
    ));
}

pub fn update_rectangles_color(mut rectangles_query: Query<(&mut Sprite, &Transform, &RectangleCollider2D)>, cursor_query: Query<(&Transform, &Cursor)>) {
    let (cursor_transform, cursor_rect) = cursor_query.single();
    let cursor_shape = create_rectangle_collision_shape(cursor_transform.translation.truncate(), cursor_rect.click_area, 0.0, false);

    for (mut sprite, transform, rectangle) in rectangles_query.iter_mut() {
        println!("quat rotation {}", transform.rotation);

        let collision_shape = create_rectangle_collision_shape(transform.translation.truncate(), rectangle.extents, transform.rotation.to_axis_angle().1, true);

        sprite.color = if convex_shapes_overlap(&collision_shape, &cursor_shape) {
            COLLIDING_COLOR
        } else {
            NOT_COLLIDING_COLOR
        }
    }
}

pub fn create_rectangle_collision_shape(center: Vec2, extents: Vec2, rotation: f32, print: bool) -> Rectangle2D {
    if print { println!("rotation {}", rotation); }

    let center = Vector2::xy(center.x, center.y);
    let extents = Vector2::xy(extents.x, extents.y);

    Rectangle2D::new(center, extents, rotation)
}