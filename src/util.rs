use bevy::{prelude::*, math::Vec3Swizzles};

use crate::level::Level;

pub type EntityQuery<'w, 's, T> = Query<'w, 's, Entity, With<T>>;

pub struct Cursor {
    pub pos: Vec2,
    pub prev_pos: Vec2,

    pub world_pos: Vec2,
    pub prev_world_pos: Vec2,

    pub moved: bool,
}

impl Default for Cursor {
    fn default() -> Self {
        Self { 
            pos: Vec2::ZERO, 
            prev_pos: Vec2::ZERO,
            world_pos: Vec2::ZERO,
            prev_world_pos: Vec2::ZERO,
            moved: false 
        }
    }
}

impl Cursor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn delta(&self) -> Vec2 {
        self.prev_pos - self.pos
    }
}

pub fn cursor_pos(
    offset: Query<&Transform, With<Camera>>,

    windows: Res<Windows>,
    mut cursor: ResMut<Cursor>,
    mut cursor_input: EventReader<CursorMoved>,
) {
    cursor.prev_pos = cursor.pos;
    cursor.prev_world_pos = cursor.world_pos;

    if let Some(cursor_moved) = cursor_input.iter().last() {        
        let window = windows.get_primary().unwrap();
        let window_size = Vec2::new(window.width(), window.height());

        let pos = cursor_moved.position - (window_size / 2.0);

        cursor.pos = pos;
        cursor.world_pos = pos + offset.single().translation.xy();

        cursor.moved = true;
    } else {
        cursor.moved = false;
    }
}