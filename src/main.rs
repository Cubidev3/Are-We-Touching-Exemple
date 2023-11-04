mod camera;
mod cursor;
mod test_rect;
mod support_circle;

use bevy::app::{App, Startup, Update};
use bevy::DefaultPlugins;
use bevy::math::Vec2;
use crate::camera::spawn_camera;
use crate::cursor::{move_cursor, my_cursor_system, MyWorldCoords, spawn_cursor};
use crate::support_circle::{spawn_support_rect, update_translation_support_rect};
use crate::test_rect::{spawn_rectangles, update_rectangles_color};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MyWorldCoords(Vec2::ZERO))
        .add_systems(Startup, (spawn_camera, spawn_cursor, spawn_rectangles, spawn_support_rect))
        .add_systems(Update, (my_cursor_system, move_cursor, update_rectangles_color))
        .add_systems(bevy::app::PostUpdate, update_translation_support_rect)
        .run();
}