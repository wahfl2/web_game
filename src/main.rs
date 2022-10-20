use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use editor::serde::SaveLoaded;
use game::{level::{level_startup, LevelEntity}, GamePlugin};
use util::{Cursor, cursor_pos};

pub mod game;
pub mod editor;
pub mod util;

pub const METERS_PER_PIXEL: f32 = 1.0 / 1000.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0 / METERS_PER_PIXEL))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(GamePlugin)
        .add_startup_system(setup)
        .add_startup_system(level_startup)
        .insert_resource(SaveLoaded(false))
        .insert_resource(LevelEntity { entity: None })
        .insert_resource(Cursor::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -750.0),
            timestep_mode: TimestepMode::Fixed { dt: 1.0 / 60.0, substeps: 16 },
            ..default()
        })
        .add_system_to_stage(CoreStage::PreUpdate, cursor_pos)
        .run();
}

pub fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera2dBundle::default());
}