use bevy::prelude::*;

use crate::editor::serde::editor_load;

use self::player::{spawn::{player_spawn, Respawn}, controls::controls::player_controls, camera::player_camera, components::*};

pub mod level;
pub mod player;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(editor_load)
            .insert_resource(Respawn(true))
            .add_system(player_spawn)
            .insert_resource(FailedShot(false))
            .add_system(player_camera)
            .add_system(player_controls);
    }
}