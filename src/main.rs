use bevy::prelude::*;
use util::{Cursor, cursor_pos};
use crate::editor::*;

pub mod level;
pub mod editor;
pub mod util;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(editor_load)
        .insert_resource(Cursor { pos: Vec2::ZERO })
        .add_system(editor)
        .add_system(editor_save)
        .add_system(cursor_pos)
        .run();
}

pub fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera2dBundle::default());
}