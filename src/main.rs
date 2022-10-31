use bevy::{prelude::*, window::{WindowPlugin, WindowMode}, app::AppExit};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use editor::{serde::SaveLoaded, editor::EditorPlugin};
use game::{level::{level_startup, LevelEntity}, GamePlugin};
use util::{Cursor, cursor_pos, preload_assets, PreloadedAssets};

pub mod game;
pub mod editor;
pub mod util;
pub mod constants;

pub const METERS_PER_PIXEL: f32 = 1.0 / 1000.0;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0 / METERS_PER_PIXEL))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(GamePlugin)
        .add_startup_system(setup)
        .add_startup_system(level_startup)
        .add_startup_system(preload_assets)
        .insert_resource(PreloadedAssets::new())
        .insert_resource(SaveLoaded(false))
        .insert_resource(LevelEntity { entity: None })
        .insert_resource(Cursor::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -750.0),
            timestep_mode: TimestepMode::Fixed { dt: 1.0 / 60.0, substeps: 16 },
            scaled_shape_subdivision: 32,
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa { samples: 4 })
        .add_system_to_stage(CoreStage::PreUpdate, cursor_pos)
        .add_system(quit)
        .run();
}

pub fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera2dBundle::default());
}

pub fn quit(
    mut exit: EventWriter<AppExit>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}