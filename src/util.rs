use std::f32::consts::PI;

use bevy::{prelude::*, math::Vec3Swizzles};

pub const DEGREES: f32 = 180.0 / PI;

pub type EntityQuery<'w, 's, T> = Query<'w, 's, Entity, With<T>>;

pub struct Cursor {
    pub pos: Vec2,
    pub prev_pos: Vec2,

    pub world_pos: Vec2,
    pub prev_world_pos: Vec2,

    pub delta: Vec2,

    pub moved: bool,
}

impl Default for Cursor {
    fn default() -> Self {
        Self { 
            pos: Vec2::ZERO, 
            prev_pos: Vec2::ZERO,
            world_pos: Vec2::ZERO,
            prev_world_pos: Vec2::ZERO,
            delta: Vec2::ZERO,
            moved: false 
        }
    }
}

impl Cursor {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn cursor_pos(
    projection: Query<&OrthographicProjection>,
    c_transform_query: Query<&GlobalTransform, With<Camera>>,

    windows: Res<Windows>,
    mut cursor: ResMut<Cursor>,
    mut cursor_input: EventReader<CursorMoved>,
) {
    cursor.prev_pos = cursor.pos;
    cursor.prev_world_pos = cursor.world_pos;

    let scale = projection.single().scale;

    if let Some(cursor_moved) = cursor_input.iter().last() {        
        let window = windows.get_primary().unwrap();
        let window_size = Vec2::new(window.width(), window.height());

        let pos = cursor_moved.position - (window_size / 2.0);

        cursor.pos = pos;
        
        let cam_translation = c_transform_query.single().compute_transform().translation;
        cursor.world_pos = (pos * scale) + cam_translation.xy();

        cursor.moved = true;
    } else {
        cursor.moved = false;
    }

    cursor.delta = (cursor.prev_pos - cursor.pos) * scale;
}

pub fn update_color_material(
    handle: &Handle<ColorMaterial>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    color: Color,
) {
    let material_op = materials.get_mut(handle);
    if let Some(material) = material_op {
        material.color = color;
    }
}