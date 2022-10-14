use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use editor::editor::EditorPlugin;
use level::{level_startup, LevelEntity};
use util::{Cursor, cursor_pos};

pub mod level;
pub mod editor;
pub mod util;

pub const METERS_PER_PIXEL: f32 = 1.0 / 100.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0 / METERS_PER_PIXEL))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(EditorPlugin)
        .add_startup_system(setup)
        .add_startup_system(level_startup)
        .insert_resource(LevelEntity { entity: None })
        .insert_resource(Cursor::default())
        .add_system_to_stage(CoreStage::PreUpdate, cursor_pos)
        .run();
}

pub fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera2dBundle::default());
}