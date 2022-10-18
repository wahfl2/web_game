use bevy::prelude::*;

use crate::editor::{serde::editor_load, camera::camera_movement};

use self::player::{spawn::player_spawn, controls::player_controls, camera::player_camera};

pub mod level;
pub mod player;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(editor_load)
            .add_startup_system(player_spawn)
            .add_system(player_camera)
            .add_system(player_controls);
    }
}