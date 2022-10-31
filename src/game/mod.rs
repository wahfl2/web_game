use bevy::prelude::*;

use crate::editor::serde::editor_load;

use self::player::{spawn::{player_spawn, Respawn}, controls::{controls::{player_controls, WebPartEntities}, web_connections::update_web_connections}, camera::player_camera, components::*, respawn_message::{spawn_message, respawn_message}};

pub mod level;
pub mod player;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(editor_load)
            .insert_resource(Respawn(true))
            .add_system(player_spawn)
            .add_startup_system(spawn_message)
            .insert_resource(FailedShot(false))
            .insert_resource(WebPartEntities { entities: Vec::new() })
            .insert_resource(FramesRestartKeyHeld(0))
            .add_system(player_camera)
            .add_system(player_controls)
            .add_system(respawn_message)
            .add_system_to_stage(CoreStage::PostUpdate, update_web_connections);
    }
}